use self::{acpi::hpet::HpetDevice, rtc::RtcDevice};

//

pub mod acpi;
pub mod pic;
pub mod pit;
pub mod ps2;
pub mod qemu;
pub mod rtc;

//

pub fn lazy_install_io_drivers() {
    *hyperion_vfs::IO_DEVICES.lock() = || {
        hyperion_vfs::install_dev("/dev/rtc", RtcDevice);
        hyperion_vfs::install_dev("/dev/hpet", HpetDevice);
    };
}