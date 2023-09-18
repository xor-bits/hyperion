#![no_std]

//

use core::fmt::{Arguments, Write};

use spin::{Lazy, Mutex};
#[cfg(not(target_arch = "x86_64"))]
use uart_16550::MmioSerialPort as SerialPort;
#[cfg(target_arch = "x86_64")]
use uart_16550::SerialPort;

//

#[cfg(not(target_arch = "x86_64"))]
const SERIAL_PORT: usize = 0x1000_0000; // serial port base addr
#[cfg(target_arch = "x86_64")]
const SERIAL_PORT: u16 = 0x3F8; // serial i/o port

//

#[inline(always)]
pub fn _print(args: Arguments) {
    _ = COM1.lock().write_fmt(args);
}

//

static COM1: Lazy<Mutex<SerialPort>> = Lazy::new(|| {
    let mut port = unsafe { SerialPort::new(SERIAL_PORT) };
    port.init();
    Mutex::new(port)
});
