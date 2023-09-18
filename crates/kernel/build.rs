use std::{env::var, error::Error};

//

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=kernel/build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");

    let kernel = var("CARGO_PKG_NAME")?;
    println!("cargo:rerun-if-env-changed=CARGO_PKG_NAME .");
    //let arch = var("CARGO_CFG_TARGET_ARCH")?;
    //println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");

    println!("cargo:rustc-link-arg=-no-pie");

    let mut bootloader: Option<&'static str> = None;
    let mut set = |s| {
        if let Some(already) = bootloader {
            println!("cargo:warning=Bootloaders {s} and {already} are mutually exclusive");
            panic!();
        } else {
            bootloader = Some(s);
        }
    };

    cfg_if::cfg_if! {
        if #[cfg(feature = "limine")] {
            println!("cargo:warning=limine");
            set("limine");
        } else if #[cfg(feature = "opensbi")] {
            println!("cargo:warning=opensbi");
            set("opensbi");
        } else if #[cfg(feature = "bootboot")] {
            println!("cargo:warning=bootboot");
            set("bootboot");
        } else if #[cfg(feature = "multiboot1")] {
            println!("cargo:warning=multiboot1");
            set("multiboot1");
        } else if #[cfg(feature = "multiboot2")] {
            println!("cargo:warning=multiboot2");
            set("multiboot2");
        } else {
            println!("cargo:warning=No bootloader specified");
        }
    }

    if let Some(bootloader) = bootloader {
        let script = format!("crates/boot-{bootloader}/src/link.ld");
        println!("cargo:rustc-link-arg-bin={kernel}=--script={script}");
        // println!("cargo:rustc-link-arg-bin={kernel}=-T");
        // println!("cargo:rustc-link-arg-bin={kernel}={script}");
        println!("cargo:rerun-if-changed={script}");
    } else {
        println!("cargo:warning=No bootloaders given");
        panic!();
    };

    Ok(())
}
