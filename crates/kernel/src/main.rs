#![doc = include_str!("../../../README.md")]
//
#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]
//
#![feature(
    const_option,
    allocator_api,
    pointer_is_aligned,
    int_roundings,
    array_chunks,
    core_intrinsics,
    custom_test_frameworks,
    panic_can_unwind
)]
#![test_runner(crate::testfw::test_runner)]
#![reexport_test_harness_main = "test_main"]

//

use hyperion_arch as arch;
use hyperion_boot as boot;
use hyperion_boot_interface::Cpu;
use hyperion_drivers as drivers;
use hyperion_kernel_info::{NAME, VERSION};
#[cfg(target_arch = "x86_64")]
use hyperion_kshell as kshell;
use hyperion_log::debug;
use hyperion_log_multi as log_multi;
use hyperion_random as random;
#[cfg(target_arch = "x86_64")]
use hyperion_scheduler as scheduler;

extern crate alloc;

//

pub mod panic;
#[cfg(target_arch = "x86_64")]
pub mod syscall;
#[cfg(test)]
pub mod testfw;

//

/// the actual entrypoint
#[no_mangle]
extern "C" fn _hyperion_start() -> ! {
    let syscon = 0x100000 as *mut u32;
    unsafe { core::ptr::write_volatile(syscon, 0x5555) };

    arch::int::disable();
    boot::init();
    kernel_main();
}

#[no_mangle]
fn kernel_main() -> ! {
    // enable logging and and outputs based on the kernel args,
    // any logging before won't be shown
    log_multi::init_logger();

    debug!("Entering kernel_main");
    debug!("{NAME} {VERSION} was booted with {}", boot::NAME);

    #[cfg(target_arch = "x86_64")]
    arch::syscall::set_handler(syscall::syscall);
    #[cfg(target_arch = "x86_64")]
    arch::early_boot_cpu();
    #[cfg(target_arch = "x86_64")]
    random::provide_entropy(&arch::rng_seed().to_ne_bytes());

    drivers::lazy_install_early();

    #[cfg(test)]
    test_main();

    // main task(s)
    #[cfg(target_arch = "x86_64")]
    scheduler::executor::spawn(kshell::kshell());

    // jumps to [smp_main] right bellow + wakes up other threads to jump there
    boot::smp_init(smp_main);
}

fn smp_main(cpu: Cpu) -> ! {
    debug!("{cpu} entering smp_main");

    #[cfg(target_arch = "x86_64")]
    arch::early_per_cpu(&cpu);

    if cpu.is_boot() {
        drivers::lazy_install_late();
        debug!("boot cpu drivers installed");
    }

    #[cfg(target_arch = "x86_64")]
    scheduler::spawn(move || {
        scheduler::executor::run_tasks();
    });
    debug!("resetting {cpu} scheduler");
    #[cfg(target_arch = "x86_64")]
    scheduler::reset();

    arch::done();
}
