#![no_std]

//

extern crate alloc;

use core::fmt::Arguments;

use arcstr::{literal, ArcStr};
use crossbeam::atomic::AtomicCell;
use hyperion_boot::args;
use hyperion_log::{set_logger, LogLevel, Logger};

//

pub fn init_logger() {
    set_logger(&MULTI_LOGGER);
    let args = args::get();
    set_qemu(args.serial_log_level);
}

pub fn set_qemu(level: LogLevel) {
    MULTI_LOGGER.qemu.store(level)
}

//

static MULTI_LOGGER: MultiLogger = MultiLogger {
    qemu: AtomicCell::new(LogLevel::Debug),
};

//

struct MultiLogger {
    qemu: AtomicCell<LogLevel>,
}

const _: () = assert!(AtomicCell::<LogLevel>::is_lock_free());

//

impl Logger for MultiLogger {
    fn is_enabled(&self, level: LogLevel) -> bool {
        self.qemu.load() >= level
    }

    fn proc_name(&self) -> Option<ArcStr> {
        if !hyperion_scheduler::running() {
            return None;
        }

        let active = hyperion_scheduler::process();

        let Some(name) = active.name.try_read() else {
            return Some(literal!("<name-locked>"));
        };

        Some(name.clone())
    }

    fn print(&self, _level: LogLevel, args: Arguments) {
        hyperion_driver_qemu::_print(args);
        // TODO: bring back fbo logging
    }
}
