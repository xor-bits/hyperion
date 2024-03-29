use alloc::vec::Vec;

use hyperion_log::error;
use hyperion_mem::to_higher_half;
use spin::{Lazy, Mutex, MutexGuard};
use x86_64::PhysAddr;

use super::{apic::ApicId, madt::MADT, ReadWrite};

//

pub static IO_APICS: Lazy<&'static [Mutex<IoApic>]> = Lazy::new(|| {
    let mut io_apics = Vec::new();

    for &info in MADT.io_apics.iter() {
        io_apics.push(Mutex::new(IoApic::init(info)));
    }

    Vec::leak(io_apics)
});

//

pub struct IoApic {
    regs: &'static mut IoApicRegs,
    // id: u8,
    // gsi_base: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct IoApicRegs {
    /// register selector
    pub register_select_register: ReadWrite,
    /// register data
    pub window_register: ReadWrite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoApicInfo {
    pub addr: u32,
    pub id: u8,
    pub gsi_base: u32,
}

//

impl IoApic {
    pub fn iter() -> impl Iterator<Item = MutexGuard<'static, IoApic>> {
        IO_APICS.iter().map(Mutex::lock)
    }

    pub fn any() -> Option<MutexGuard<'static, IoApic>> {
        IO_APICS.iter().next().map(Mutex::lock)
    }

    pub fn init(IoApicInfo { addr, .. }: IoApicInfo) -> Self {
        hyperion_log::trace!("I/O APIC init");

        let addr = to_higher_half(PhysAddr::new(addr as u64));

        Self {
            regs: unsafe { &mut *addr.as_mut_ptr() },
            // id,
            // gsi_base,
        }
    }

    pub fn set_irq_any(&mut self, io_apic_irq: u8, irq: u8) -> ApicId {
        let io_apic_irq_router = ApicId::iter()
            .find(|id| id.is_ioapic_compatible())
            .unwrap_or_else(|| {
                error!(
                    "No suitable APICs for handling I/O APIC interrupts, using fallback LAPIC 0"
                );
                ApicId::new(0)
            });

        self.set_irq(io_apic_irq, io_apic_irq_router, irq);
        io_apic_irq_router
    }

    // https://wiki.osdev.org/APIC#IO_APIC_Registers
    pub fn set_irq(&mut self, io_apic_irq: u8, apic: ApicId, irq: u8) {
        if apic.inner() > 0xFF {
            panic!("APIC ID `{apic:?}` too large");
        }

        // the value is split to 2 32 bit registers
        let low_index: u32 = 0x10 + ((io_apic_irq as u32) * 2);
        let high_index: u32 = low_index + 1;

        let mut high = self.read(high_index);
        high &= !0xff000000;
        high |= apic.inner() << 24;
        self.write(high_index, high);

        let mut low = self.read(low_index);
        // Clear mask, enabling this interrupt
        low &= !(1 << 16);
        // Use physical destination mode, not logical destination mode
        low &= !(1 << 11);
        // Set the delivery mode to Fixed
        low &= !0x700;
        // Set the lowest 8 bits, which correspond to the IRQ vector.
        low &= !0xff;
        low |= irq as u32;
        self.write(low_index, low);
    }

    fn read(&mut self, reg: u32) -> u32 {
        self.regs.register_select_register.write(reg);
        self.regs.window_register.read()
    }

    fn write(&mut self, reg: u32, val: u32) {
        self.regs.register_select_register.write(reg);
        self.regs.window_register.write(val);
    }
}

/* impl IoApicRegs {
    pub fn
} */
