#![no_std]

//

#[cfg(feature = "limine")]
pub use hyperion_boot_limine::*;
#[cfg(feature = "opensbi")]
pub use hyperion_boot_opensbi::*;

//

pub mod args;

/* #[cfg(feature = "multiboot1")]
#[path = "multiboot1/mod.rs"]
#[allow(clippy::module_inception)]
mod boot;
#[cfg(feature = "multiboot2")]
#[path = "multiboot2/mod.rs"]
#[allow(clippy::module_inception)]
mod boot;
#[cfg(feature = "bootboot")]
#[path = "bootboot/mod.rs"]
#[allow(clippy::module_inception)]
mod boot;
#[cfg(feature = "limine")]
#[path = "limine/mod.rs"]
#[allow(clippy::module_inception)]
mod boot; */
