#![no_std]

//

#[cfg(target_arch = "riscv64")]
pub use hyperion_arch_riscv64::*;
#[cfg(target_arch = "x86_64")]
pub use hyperion_arch_x86_64::*;
