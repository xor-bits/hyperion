use core::panic::PanicInfo;

use hyperion_arch::done;
use hyperion_log::println;

//

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    panic_unwind(info);
    done();
}

#[cfg(test)]
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    panic_unwind(info);
    crate::testfw::test_panic_handler(info);
    done();
}

fn panic_unwind(info: &PanicInfo) {
    println!("Kernel CPU {info}");
    // hyperion_backtrace::print_backtrace();
}
