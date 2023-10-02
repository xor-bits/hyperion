use core::sync::atomic::{AtomicBool, Ordering};

use hyperion_mem::pmm;
use x86_64::{structures::tss::TaskStateSegment, VirtAddr};

//

#[derive(Debug)]
pub struct Tss {
    pub inner: TaskStateSegment,
    pub stacks: TssStacks,
}

#[derive(Debug)]
pub struct TssStacks {
    interrupt: [AtomicBool; 7],
    // privilege: [bool; 3],
}

//

impl Tss {
    pub fn new() -> Self {
        let mut tss = Self {
            inner: TaskStateSegment::new(),
            stacks: TssStacks {
                interrupt: [(); 7].map(|_| AtomicBool::new(false)),
                // privilege: [false; 3],
            },
        };

        // TODO: (2 unused stacks) privilege stack 0 could reuse the boot stack?
        // just like the kernel stack that the syscall handler switches to
        //
        // so the syscall handler should switch to this stack here, which
        // should be the stack that the bootloader gave
        tss.add_priv(0);

        tss.add_int(0);
        tss.add_int(1);

        hyperion_log::debug!("TSS: {tss:?}");

        tss
    }

    fn add_int(&mut self, idx: usize) {
        let stack = Self::alloc_stack();
        self.inner.interrupt_stack_table[idx] = VirtAddr::from_ptr(stack.as_ptr_range().end);
        self.stacks.interrupt[idx].store(true, Ordering::SeqCst);
    }

    fn add_priv(&mut self, idx: usize) {
        let stack = Self::alloc_stack();
        self.inner.privilege_stack_table[idx] = VirtAddr::from_ptr(stack.as_ptr_range().end);
        // self.stacks.privilege[idx] = true;
    }

    fn alloc_stack() -> &'static mut [u8] {
        pmm::PFA.alloc(8).leak()
    }
}

impl Default for Tss {
    fn default() -> Self {
        Self::new()
    }
}

impl TssStacks {
    pub fn take_interrupt_stack(&self) -> Option<u16> {
        self.interrupt
            .iter()
            .enumerate()
            .find_map(|(idx, avail)| avail.swap(false, Ordering::SeqCst).then_some(idx as u16))
    }
}
