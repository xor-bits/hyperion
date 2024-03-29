//! Physical memory management
//!
//! Page frame allocating

use alloc::vec::Vec;
use core::{
    alloc::{AllocError, Allocator, Layout},
    fmt,
    hint::black_box,
    intrinsics::volatile_copy_nonoverlapping_memory,
    mem::{self, transmute, MaybeUninit},
    ptr::{self, NonNull},
    slice,
    sync::atomic::{AtomicU16, AtomicUsize, Ordering},
};

use hyperion_boot::memmap;
use hyperion_boot_interface::Memmap;
use hyperion_log::debug;
use hyperion_num_postfix::NumberPostfix;
use spin::Lazy;
use x86_64::{
    align_up,
    structures::paging::{Page, PhysFrame},
    PhysAddr, VirtAddr,
};

use super::{from_higher_half, to_higher_half};

//

pub static PFA: Lazy<PageFrameAllocator> = Lazy::new(PageFrameAllocator::init);

const PAGE_SIZE: usize = 0x1000; // 4KiB pages

//

#[derive(Debug)]
struct TooManyRefs;

//

pub struct PageInfo {
    ref_count: AtomicU16,
}

impl PageInfo {
    fn alloc(&self) -> bool {
        self.ref_count
            .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    /// # Safety
    /// this page should not be deallocated during this clone, it can be cloned though
    unsafe fn copy(&self) -> Result<(), TooManyRefs> {
        // TODO: orderings
        self.ref_count
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |old| old.checked_add(1))
            .map(|_| {})
            .map_err(|_| TooManyRefs)
    }

    /// returns true if the page is actually free to allocate now
    fn free(&self) -> bool {
        match self.ref_count.fetch_sub(1, Ordering::Release) {
            0 => panic!("double free detected"),
            1 => true,
            _ => false,
        }
    }
}

//

pub struct PageFrameAllocator {
    pages: &'static [PageInfo],
    // bitmap: AtomicBitmap<'static>,
    usable: AtomicUsize,
    used: AtomicUsize,
    total: AtomicUsize,

    last_alloc_index: AtomicUsize,
}

#[derive(Debug)]
pub struct PageFrame {
    first: PhysAddr,
    count: usize,
}

//

impl PageFrameAllocator {
    /// System total memory in bytes
    pub fn total_mem(&self) -> usize {
        self.total.load(Ordering::SeqCst)
    }

    /// System usable memory in bytes
    pub fn usable_mem(&self) -> usize {
        self.usable.load(Ordering::SeqCst)
    }

    /// Currently used usable memory in bytes
    pub fn used_mem(&self) -> usize {
        self.used.load(Ordering::SeqCst)
    }

    /// Currently free usable memory in bytes
    pub fn free_mem(&self) -> usize {
        self.usable_mem() - self.used_mem()
    }

    /// Reserved memory in bytes
    pub fn reserved_mem(&self) -> usize {
        self.total_mem() - self.usable_mem()
    }

    pub fn bitmap_len(&self) -> usize {
        self.pages.len()
    }

    /// # Safety
    ///
    /// this is safe to call once the bootloader memory is guaranteed to not be used anymore
    ///
    /// so after the bootloader stacks are freed and the bootloader page mapper is freed
    /// and there are no calls to things like Limine requests
    ///
    /// I use Lazy in limine requests to avoid reading the raw data twice, so most Limine
    /// requests should be already cached, and 'should be' is admittedly not 'guaranteed'
    pub unsafe fn free_bootloader(&self) {
        let reclaimable: Vec<Memmap> = hyperion_boot::memmap()
            .filter(|map| map.is_bootloader_reclaimable())
            .collect();

        // the vec has to be collected because the memmaps are stored in the bootloader reclaimable memory
        let reclaimable = black_box(reclaimable);

        for region in reclaimable {
            self.free_memmap(region);
        }
    }

    /// Free up pages
    pub fn free(&self, mut frame: PageFrame) {
        if frame.first.as_u64() == 0 || frame.count == 0 {
            panic!();
        }

        frame.as_bytes_mut().fill(0);
        self.free_no_overwrite(frame);
    }

    /// Free up pages without destroying the data
    pub fn free_no_overwrite(&self, frame: PageFrame) {
        if frame.first.as_u64() == 0 || frame.count == 0 {
            panic!();
        }

        let page = frame.first.as_u64() as usize / PAGE_SIZE;
        // debug!(
        //     "freeing pages first={page} count={} from={}",
        //     frame.count,
        //     core::panic::Location::caller()
        // );
        for page in page..page + frame.count {
            if self.pages[page].free() {
                self.used.fetch_sub(PAGE_SIZE, Ordering::Release);
            }
        }
    }

    /// mark a page as shared (or make a copy if it if there are too many refs)
    ///
    // /// `Ok` means that the frame is the same
    // /// `Err` means that a copy was made
    ///
    /// # Safety
    /// the pages should not be modified or deallocated during the copy
    ///
    /// the original and copied pages shouldn't be written to
    ///
    /// `mapped` should point to `frame` in the active page mapper
    pub unsafe fn fork(&self, frame: PhysFrame, mapped: Page) -> PhysFrame {
        if frame.start_address().as_u64() == 0 {
            panic!();
        }
        let page = frame.start_address().as_u64() as usize / PAGE_SIZE;

        if matches!(unsafe { self.pages[page].copy() }, Err(TooManyRefs)) {
            unsafe { self.cold_copy_fork(mapped) }
        } else {
            frame
        }
    }

    #[cold]
    unsafe fn cold_copy_fork(&self, mapped: Page) -> PhysFrame {
        let copy = self.alloc(1);
        unsafe {
            volatile_copy_nonoverlapping_memory::<u8>(
                copy.virtual_addr().as_mut_ptr(),
                mapped.start_address().as_ptr(),
                PAGE_SIZE,
            );
        }
        PhysFrame::from_start_address(copy.physical_addr()).unwrap()
    }

    /// handle a page fault from a forked CoW page
    ///
    // /// `Ok` means that the frame is still the same
    // /// `Err` means that a copy was made
    ///
    /// # Internal
    ///
    /// if the page has 0 refs, calling this panics
    ///
    /// if the page has 1 ref, it is now exclusive and can just be made writeable
    ///
    /// if the page has 2 or more refs,
    /// the ref count is decremented and a copy is made and that copy is returned
    ///
    /// # Safety
    /// `mapped` should point to `frame` in the active page mapper
    pub unsafe fn fork_page_fault(&self, frame: PhysFrame, mapped: Page) -> PhysFrame {
        if frame.start_address().as_u64() == 0 {
            panic!();
        }
        let page = frame.start_address().as_u64() as usize / PAGE_SIZE;
        let ref_count = &self.pages[page].ref_count;

        match ref_count.load(Ordering::Acquire) {
            0 => {
                // trying to fork a free page
                panic!()
            }
            1 => {
                // exclusive access
                frame
            }
            mut other => {
                // make a copy of the original page,
                // before giving up the ref
                let copy = self.alloc(1);
                unsafe {
                    volatile_copy_nonoverlapping_memory::<u8>(
                        copy.virtual_addr().as_mut_ptr(),
                        mapped.start_address().as_ptr(),
                        PAGE_SIZE,
                    );
                }

                loop {
                    // decrement the ref count and copy the page
                    // fetch_sub won't work because if the second 'owner' frees it
                    // (right after the load above),
                    // the ref count becomes 1 and the fetch_sub would mark the page as free,
                    // which is obviously bad
                    if ref_count
                        .compare_exchange(other, other - 1, Ordering::Release, Ordering::Relaxed)
                        .is_ok()
                    {
                        if other == 1 {
                            // 2 copies were made and the original got deallocated
                            // (a small waste of time but idc)
                            self.used.fetch_sub(PAGE_SIZE, Ordering::Release);
                        }

                        // a copy has been made and the original page is now
                        // either deallocated or shared between some other process(es)
                        return PhysFrame::from_start_address(copy.physical_addr()).unwrap();
                    } else {
                        // it doesnt matter if the ref count goes to 1 here
                        // because the copy has already been made
                        other = ref_count.load(Ordering::Acquire);
                    }
                }
            }
        }
    }

    /// Alloc pages
    ///
    /// Use [`Self::free`] to not leak pages (-> memory)
    pub fn alloc(&self, count: usize) -> PageFrame {
        if count == 0 {
            return PageFrame {
                first: PhysAddr::new(0),
                count: 0,
            };
        };

        // hyperion_log::debug!(
        //     "allocating {count} pages (from {})",
        //     core::panic::Location::caller()
        // );

        // TODO: lock-less page alloc
        let from = self.last_alloc_index.load(Ordering::SeqCst);
        let first_page = self.alloc_at(from, count).unwrap_or_else(|| {
            // TODO: handle OOM a bit better
            self.last_alloc_index.store(0, Ordering::SeqCst);
            self.alloc_at(0, count).expect("OOM")
        });

        self.last_alloc_index
            .store(first_page + count, Ordering::SeqCst);

        let addr = PhysAddr::new((first_page * PAGE_SIZE) as u64);
        let page_ptr: *mut MaybeUninit<u8> = to_higher_half(addr).as_mut_ptr();
        assert!(
            page_ptr.is_aligned_to(PAGE_SIZE),
            "pages should be aligned to {PAGE_SIZE}"
        );

        // Safety: the pages get protected from allocations
        let page_data: &mut [MaybeUninit<u8>] =
            unsafe { slice::from_raw_parts_mut(page_ptr, count * PAGE_SIZE) };
        /* let page_data = */
        fill_maybeuninit_slice(page_data, 0);

        self.used.fetch_add(count * PAGE_SIZE, Ordering::Release);

        PageFrame { first: addr, count }
    }

    // returns the page index, not the page address
    fn alloc_at(&self, from: usize, count: usize) -> Option<usize> {
        // let mut first_page = self.last_alloc_index.load(Ordering::SeqCst);

        let mut first_page = from;
        let total_pages = self.pages.len();

        'main: loop {
            if first_page + count > total_pages {
                return None;
            }

            // TODO: lock the pages in reverse as a small optimization
            for offs in 0..count {
                let page = first_page + offs;

                // lock the window
                if !self.pages[page].alloc() {
                    // could not alloc
                    // 0..offs, means that the last page that we couldn't acquire, won't be freed
                    for offs in 0..offs {
                        let page = first_page + offs;
                        // the first swap already acquired exclusive access to these pages
                        // so they are safe to free
                        // TODO: unsafe fn
                        self.pages[page].free();
                    }

                    first_page = first_page + offs + 1;
                    continue 'main;
                }
            }

            return Some(first_page);
        }
    }

    fn free_memmap(&self, Memmap { base, len, ty }: Memmap) {
        debug!(
            "Free {ty:?} pages: [ {:#018x?} ] ({}B)",
            base..base + len,
            len.postfix_binary()
        );

        // base and len are guaranteed to be page aligned
        let frame = unsafe { PageFrame::new(PhysAddr::new(base as _), len >> 12) };
        // TODO: user space apps could read what the bootloader stored, but is it a problem?
        self.free_no_overwrite(frame);
    }

    fn init() -> Self {
        // usable system memory
        let mut usable: usize = memmap()
            .filter(|m| m.is_usable() | m.is_bootloader_reclaimable())
            .map(|Memmap { len, .. }| len)
            .sum();

        // total system memory
        let total: usize = memmap()
            .map(|Memmap { base, len, ty: _ }| base + len)
            .max()
            .unwrap_or(0);

        // the end of the usable physical memory address space
        let top = memmap()
            .filter(|m| m.is_usable() | m.is_bootloader_reclaimable())
            .map(|Memmap { base, len, ty: _ }| base + len)
            .max()
            .unwrap_or(0);

        /* // size in bytes
        let bitmap_size: usize = align_up((top / PAGE_SIZE / 8) as _, PAGE_SIZE as _) as _;
        let bitmap_data: usize = memmap()
            .filter(Memmap::is_usable)
            .find(|Memmap { len, .. }| *len >= bitmap_size)
            .expect("No place to store PageFrameAllocator bitmap")
            .base;
        let bitmap_ptr: *mut MaybeUninit<u8> =
            to_higher_half(PhysAddr::new(bitmap_data as _)).as_mut_ptr();

        // SAFETY: this bitmap is going to be initialized before it is read from
        // the memory region also gets protected from allocations
        let bitmap: &mut [MaybeUninit<u8>] =
            unsafe { slice::from_raw_parts_mut(bitmap_ptr, bitmap_size as _) };
        let bitmap = fill_maybeuninit_slice(bitmap, 0xFF);
        let bitmap = AtomicBitmap::from_mut(bitmap);
        bitmap.fill(true, Ordering::SeqCst);
        // bitmap.fill(true, Ordering::SeqCst); // initialized here
        usable -= bitmap_size; */

        let pages_len: usize = top / PAGE_SIZE;
        let pages_bytes = align_up(
            (pages_len * mem::size_of::<PageInfo>()) as _,
            PAGE_SIZE as _,
        ) as _;

        let pages_data: usize = memmap()
            .filter(Memmap::is_usable)
            .find(|Memmap { len, .. }| *len >= pages_bytes)
            .expect("Not enough contiguous memory")
            .base;
        let pages_ptr: *mut MaybeUninit<PageInfo> =
            to_higher_half(PhysAddr::new(pages_data as _)).as_mut_ptr();
        let pages = unsafe { slice::from_raw_parts_mut(pages_ptr, pages_len) };
        let pages = fill_maybeuninit_slice_with(pages, || PageInfo {
            ref_count: AtomicU16::new(1),
        });

        usable -= pages_bytes;

        let pfa = Self {
            pages,

            usable: usable.into(),
            used: usable.into(),
            total: total.into(),

            last_alloc_index: 0.into(),
        };

        // free up some pages
        hyperion_log::debug!("PMM uses {}B to track pages", pages_bytes.postfix_binary());
        for mut region in memmap().filter(Memmap::is_usable) {
            if region.base == pages_data {
                // skip the previously allocated `pages` spot
                region.base += pages_bytes;
                region.len -= pages_bytes;
            }
            if region.len == 0 {
                continue;
            }

            pfa.free_memmap(region);
        }

        debug!("PFA initialized:\n{pfa}");

        pfa
    }
}

unsafe impl Allocator for PageFrameAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let frame = self.alloc(layout.size() / PAGE_SIZE);

        NonNull::new(frame.virtual_addr().as_mut_ptr())
            .map(|first| NonNull::slice_from_raw_parts(first, frame.byte_len()))
            .ok_or(AllocError)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        self.free(PageFrame {
            first: from_higher_half(VirtAddr::new(ptr.as_ptr() as u64)),
            count: layout.size() / PAGE_SIZE,
        })
    }
}

impl fmt::Display for PageFrameAllocator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Total system memory: {}B",
            self.total_mem().postfix_binary()
        )?;
        writeln!(
            f,
            "Usable system memory: {}B",
            self.usable_mem().postfix_binary()
        )?;
        writeln!(
            f,
            "Used system memory: {}B",
            self.used_mem().postfix_binary()
        )?;
        writeln!(
            f,
            "Free system memory: {}B",
            self.free_mem().postfix_binary()
        )?;
        write!(
            f,
            "Reserved system memory: {}B",
            self.reserved_mem().postfix_binary()
        )?;

        Ok(())
    }
}

impl PageFrame {
    /// # Safety
    ///
    /// The caller has to make sure that it has exclusive access to bytes in physical memory range
    /// `first..first + PAGE_SIZE * count`
    pub const unsafe fn new(first: PhysAddr, count: usize) -> Self {
        Self { first, count }
    }

    /// physical address of the first page
    pub const fn physical_addr(&self) -> PhysAddr {
        self.first
    }

    pub fn virtual_addr(&self) -> VirtAddr {
        to_higher_half(self.first)
    }

    /// number of pages
    pub const fn len(&self) -> usize {
        self.count
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// number of bytes
    pub const fn byte_len(&self) -> usize {
        self.count * PAGE_SIZE
    }

    pub fn as_bytes(&self) -> &[u8] {
        let addr = self.virtual_addr().as_mut_ptr();

        // Safety:
        // &mut self makes sure that this is the only safe mut ref
        // The page frame allocator gave exclusive access to these bytes
        unsafe { slice::from_raw_parts(addr, self.byte_len()) }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        let addr = self.virtual_addr().as_mut_ptr();

        // Safety:
        // The page frame allocator gave exclusive access to these bytes
        unsafe { slice::from_raw_parts_mut(addr, self.byte_len()) }
    }

    /// Leak the PageFrame to get a static mut ref to it
    ///
    /// # Note
    ///
    /// page frames are not deallocated automatically anyways,
    /// this just takes ownership to give a safe method of getting a static ref to the data
    pub fn leak(mut self) -> &'static mut [u8] {
        unsafe { transmute(self.as_bytes_mut()) }
    }

    pub fn free(self) {
        PFA.free(self)
    }
}

//

fn fill_maybeuninit_slice<T: Copy>(s: &mut [MaybeUninit<T>], v: T) -> &mut [T] {
    for target in s.iter_mut() {
        unsafe { ptr::write_volatile(target, MaybeUninit::new(v)) };
    }

    // Safety: The whole slice has been filled with copies of `v`
    unsafe { MaybeUninit::slice_assume_init_mut(s) }
}

fn fill_maybeuninit_slice_with<T>(s: &mut [MaybeUninit<T>], v: impl Fn() -> T) -> &mut [T] {
    for target in s.iter_mut() {
        unsafe { ptr::write_volatile(target, MaybeUninit::new(v())) };
    }

    // Safety: The whole slice has been filled with copies of `v`
    unsafe { MaybeUninit::slice_assume_init_mut(s) }
}

#[cfg(test)]
mod tests {

    /* #[test]
    fn pfa_simple() {
        let a = PFA.alloc(1);
        assert_ne!(a.physical_addr().as_u64(), 0);

        let b = PFA.alloc(1);
        assert_ne!(b.physical_addr().as_u64(), 0);
        assert_ne!(a.physical_addr().as_u64(), b.physical_addr().as_u64());

        PFA.free(a);
        PFA.alloc_from(0);
        let c = PFA.alloc(1);
        assert_ne!(c.physical_addr().as_u64(), 0);
        assert_ne!(b.physical_addr().as_u64(), c.physical_addr().as_u64());

        let d = PFA.alloc(1);
        assert_ne!(d.physical_addr().as_u64(), 0);
        assert_ne!(c.physical_addr().as_u64(), d.physical_addr().as_u64());

        // PFA.free(a); // <- compile error as expected
        PFA.free(b);
        PFA.free(c);
        PFA.free(d);
    } */
}
