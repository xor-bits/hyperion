#![no_std]

//

use core::sync::atomic::{AtomicBool, Ordering};

pub use hyperion_clock as clock;
#[cfg(target_arch = "x86_64")]
pub use hyperion_driver_acpi as acpi;
pub use hyperion_driver_framebuffer as fbo;
// pub use hyperion_driver_pic as pic;
// pub use hyperion_driver_pit as pit;
#[cfg(target_arch = "x86_64")]
pub use hyperion_driver_rtc as rtc;
pub use hyperion_vfs as vfs;

//

pub fn lazy_install_early() {
    static ONCE: AtomicBool = AtomicBool::new(true);
    if !ONCE.swap(false, Ordering::Relaxed) {
        return;
    }

    vfs::device::set_io_device_loader(|| {
        #[cfg(target_arch = "x86_64")]
        vfs::install_dev("/dev/rtc", rtc::RtcDevice);
        #[cfg(target_arch = "x86_64")]
        vfs::install_dev("/dev/hpet", acpi::hpet::HpetDevice);
        vfs::install_dev("/dev/fbo", fbo::FboDevice);
    });

    clock::set_source_picker(|| {
        // TODO: more clocks
        cfg_if::cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                // Some(&*pit::PIT)
                Some(&*acpi::hpet::HPET)
            } else if #[cfg(target_arch = "riscv64")] {
                None
            } else {
                None
            }
        }
    });
}

pub fn lazy_install_late() {
    static ONCE: AtomicBool = AtomicBool::new(true);
    if !ONCE.swap(false, Ordering::Relaxed) {
        return;
    }

    hyperion_keyboard::force_init_queue();
    #[cfg(target_arch = "x86_64")]
    {
        hyperion_driver_ps2::keyboard::init();
        hyperion_driver_ps2::mouse::init();
    }
}
