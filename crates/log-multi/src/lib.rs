#![no_std]

//

use core::fmt::Arguments;

use crossbeam::atomic::AtomicCell;
use hyperion_boot::args;
use hyperion_log::{set_logger, LogLevel, Logger};

//

pub fn init_logger() {
    set_logger(&MULTI_LOGGER);
    // let args = args::get();
    set_qemu(LogLevel::Trace);
    /* set_qemu(args.serial_log_level);
    set_fbo(args.video_log_level); */
}

pub fn set_qemu(level: LogLevel) {
    MULTI_LOGGER.qemu.store(level)
}

pub fn set_fbo(level: LogLevel) {
    MULTI_LOGGER.fbo.store(level)
}

//

static MULTI_LOGGER: MultiLogger = MultiLogger {
    qemu: AtomicCell::new(LogLevel::Debug),
    fbo: AtomicCell::new(LogLevel::None),
};

//

struct MultiLogger {
    qemu: AtomicCell<LogLevel>,
    fbo: AtomicCell<LogLevel>,
}

const _: () = assert!(AtomicCell::<LogLevel>::is_lock_free());

//

impl Logger for MultiLogger {
    fn is_enabled(&self, level: LogLevel) -> bool {
        self.qemu.load() >= level
    }

    fn print(&self, _level: LogLevel, args: Arguments) {
        hyperion_driver_qemu::_print(args);
        // TODO: bring back fbo logging
    }
}
