use core::panic::PanicInfo;

//

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    hyperion_log::println!("Kernel CPU {info}");
    // hyperion_backtrace::print_backtrace();
    hyperion_arch::done();
}

#[cfg(test)]
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    crate::testfw::test_panic_handler(info);
}
