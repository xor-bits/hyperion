#![no_std]

extern crate alloc;

//

pub use hyperion_boot_limine::*;

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

//

const _: () = assert!(BOOT_STACK_SIZE.rem_euclid(0x1000) == 0);
