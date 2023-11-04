use alloc::boxed::Box;
use core::mem::MaybeUninit;

use hyperion_log::trace;
use spin::{Mutex, MutexGuard};

use self::{gdt::Gdt, idt::Idt, tss::Tss};
use crate::{
    cpu_id,
    tls::{self, ThreadLocalStorage},
};

//

pub mod gdt;
pub mod idt;
pub mod ints;
pub mod tss;

//

pub fn init() {
    let cpu_id = cpu_id();
    trace!("Loading CpuState for CPU-{cpu_id}");
    let tls = if cpu_id == 0 {
        // boot cpu doesn't need to allocate
        CpuState::new_boot_tls()
    } else {
        // other cpus have to allocate theirs
        CpuState::new_tls()
    };

    tls::init(tls);
}

//

#[derive(Debug, Clone, Copy)]
pub struct CpuState {
    pub tss: &'static Tss,
    pub gdt: &'static Gdt,
    pub idt: &'static Idt,
}

type CpuDataAlloc = (
    MaybeUninit<Tss>,
    MaybeUninit<Gdt>,
    MaybeUninit<Idt>,
    MaybeUninit<ThreadLocalStorage>,
);

impl CpuState {
    fn new_boot_tls() -> &'static ThreadLocalStorage {
        static BOOT_DATA: Mutex<CpuDataAlloc> = Mutex::new(CpuState::new_uninit());

        let lock = BOOT_DATA
            .try_lock()
            .expect("boot cpu structures already initialized");

        Self::from_uninit(MutexGuard::leak(lock))
    }

    fn new_tls() -> &'static ThreadLocalStorage {
        // SAFETY: assume_init is safe, because each CpuDataAlloc field is MaybeUninit
        let data = unsafe { Box::<CpuDataAlloc>::new_uninit().assume_init() };

        Self::from_uninit(Box::leak(data))
    }

    const fn new_uninit() -> CpuDataAlloc {
        (
            MaybeUninit::uninit(),
            MaybeUninit::uninit(),
            MaybeUninit::uninit(),
            MaybeUninit::uninit(),
        )
    }

    fn from_uninit((tss, gdt, idt, tls): &'static mut CpuDataAlloc) -> &'static ThreadLocalStorage {
        let tss = tss.write(Tss::new());
        let gdt = gdt.write(Gdt::new(tss));
        gdt.load();
        let idt = idt.write(Idt::new(tss));
        idt.load();

        ThreadLocalStorage::init(tls)
    }
}
