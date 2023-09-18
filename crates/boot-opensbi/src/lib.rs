#![no_std]
#![feature(never_type)]

use hyperion_boot_interface::{Cpu, FramebufferCreateInfo};

//

pub static NAME: &str = "Nothing";

//

pub fn cmdline() -> Option<&'static str> {
    None
}

pub fn framebuffer() -> Option<FramebufferCreateInfo> {
    None
}

pub fn smp_init(f: impl Fn(Cpu) -> !) -> ! {
    loop {
        unsafe { core::arch::asm!("wfi") }
    }
}

pub fn init() {}

#[no_mangle]
#[link_section = ".boot"]
extern "C" fn _boot_main() -> ! {
    loop {
        let syscon = 0x100000 as *mut u32;
        // unsafe { core::ptr::write_volatile(syscon, 0x5555) };
        unsafe { *syscon = 0x5555 };
        unsafe { core::arch::asm!("wfi") }
    }
}

/* extern "C" {
    fn _hyperion_start();
} */
