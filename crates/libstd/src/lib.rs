#![no_std]
#![feature(
    format_args_nl,
    slice_internals,
    new_uninit,
    const_slice_from_raw_parts_mut,
    const_mut_refs
)]

//

extern crate alloc as core_alloc;

use core::fmt::{self, Write};

use hyperion_syscall::exit;

use self::fs::STDOUT;

//

pub mod sys {
    pub use hyperion_syscall::*;
}

pub mod alloc;
pub mod fs;
pub mod io;
pub mod thread;

//

#[macro_export]
macro_rules! println {
    ($($v:tt)*) => {
        $crate::_print(format_args_nl!($($v)*))
    };
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut stdout = STDOUT.lock();
    _ = stdout.write_fmt(args);
}

//

#[no_mangle]
extern "C" fn _start(a0: usize) -> ! {
    extern "Rust" {
        fn main(a: CliArgs);
    }

    unsafe {
        main(CliArgs {
            hyperion_cli_args_ptr: a0,
        });
    }

    exit(0);
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    println!("{info}");
    exit(-1);
}

//

#[derive(Clone, Copy)]
pub struct CliArgs {
    hyperion_cli_args_ptr: usize,
}

impl CliArgs {
    pub fn iter(self) -> impl Iterator<Item = &'static str> + Clone + DoubleEndedIterator {
        let mut ptr = self.hyperion_cli_args_ptr;

        let argc: usize = Self::pop(&mut ptr);
        let mut arg_lengths = ptr;
        let mut arg_strings = ptr + argc * core::mem::size_of::<usize>();

        (0..argc).map(move |_| {
            let len: usize = Self::pop(&mut arg_lengths);
            let str: &[u8] = unsafe { core::slice::from_raw_parts(arg_strings as _, len as _) };
            arg_strings += len;

            unsafe { core::str::from_utf8_unchecked(str) }
        })
    }

    fn pop<T: Sized>(top: &mut usize) -> T {
        let v = unsafe { ((*top) as *const T).read() };
        *top += core::mem::size_of::<T>();
        v
    }
}

impl fmt::Debug for CliArgs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
