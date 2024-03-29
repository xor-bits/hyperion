#![no_std]

//

use hyperion_interrupts::{IntController, INT_CONTROLLER, INT_EOI_HANDLER};
use hyperion_log::debug;
use spin::Mutex;
use x86_64::instructions::{interrupts::without_interrupts, port::Port};

//

pub static PICS: Mutex<Pics> = Mutex::new(Pics::new());

const ICW1_ICW4: u8 = 0x01; // ICW4 will be present
const ICW1_INIT: u8 = 0x10; // Init cmd

const ICW4_8086: u8 = 0x01; // 8086 mode

const EOI: u8 = 0x20;

//

pub struct Pics {
    master: Pic,
    slave: Pic,
    state: PicsState,
}

pub struct Pic {
    // IDT offset
    offs: u8,
    cmd: Port<u8>,
    data: Port<u8>,
}

#[derive(PartialEq, Eq)]
enum PicsState {
    Unknown,
    Disabled,
    Enabled,
}

//

impl Pics {
    pub const fn new() -> Self {
        let offs = 0x20;
        Self {
            master: Pic {
                offs,
                cmd: Port::new(0x20),
                data: Port::new(0x21),
            },
            slave: Pic {
                offs: offs + 8,
                cmd: Port::new(0xA0),
                data: Port::new(0xA1),
            },
            state: PicsState::Unknown,
        }
    }

    pub fn init(&mut self) {
        if self.state == PicsState::Enabled {
            return;
        }

        self.disable();

        /* // save masks
        let original_masks = [self.master.read_mask(), self.slave.read_mask()];
        debug!("masks {:?}", original_masks); */

        // ICW1: init
        // (ICW = Initialization Command Word)
        self.master.cmd(ICW1_INIT | ICW1_ICW4);
        self.slave.cmd(ICW1_INIT | ICW1_ICW4);

        // ICW2: IDT offsets
        self.master.data(self.master.offs);
        self.slave.data(self.slave.offs);

        // ICW3: cascade
        self.master.data(4);
        self.slave.data(2);

        // ICW4: cascade
        self.master.data(ICW4_8086);
        self.slave.data(ICW4_8086);

        debug!("8086 PIC initialized");
        INT_EOI_HANDLER.store(|irq| PICS.lock().end_of_interrupt(irq));
        INT_CONTROLLER.store(IntController::Pic);

        self.enable();

        /* // restore masks
        self.master.write_mask(original_masks[0]);
        self.slave.write_mask(original_masks[1]); */
    }

    pub fn mask(&mut self, irq: u8) {
        let (pic, irq) = if irq < 8 {
            (&mut self.master, irq)
        } else {
            (&mut self.slave, irq - 8)
        };

        let mask = pic.read_mask();
        pic.write_mask(mask | (1 << irq));
    }

    pub fn unmask(&mut self, irq: u8) {
        let (pic, irq) = if irq < 8 {
            (&mut self.master, irq)
        } else {
            (&mut self.slave, irq - 8)
        };

        let mask = pic.read_mask();
        pic.write_mask(mask & !(1 << irq));
    }

    pub fn enable(&mut self) {
        if self.state == PicsState::Enabled {
            return;
        }

        self.master.write_mask(0);
        self.slave.write_mask(0);
        debug!("8086 PIC enabled");
        self.state = PicsState::Enabled;
    }

    pub fn disable(&mut self) {
        if self.state == PicsState::Disabled {
            return;
        }

        self.master.write_mask(0xFF);
        self.slave.write_mask(0xFF);
        debug!("8086 PIC disabled");
        self.state = PicsState::Disabled;
    }

    pub fn end_of_interrupt(&mut self, int_id: u8) {
        if self.master.has(int_id) {
            self.master.cmd(EOI);
        }
        if self.slave.has(int_id) {
            self.slave.cmd(EOI);
        }
    }

    pub fn read_pit_count() {
        without_interrupts(|| {
            let mut port = Port::new(0x43);

            unsafe {
                port.write(0x00u8);
            }
        });
    }

    /* pub fn pit_tick() {
        PIT_CLOCK.fetch_add(1, Ordering::SeqCst);
    }

    pub fn pit_wait() -> usize {
        PIT_CLOCK.store(0, Ordering::SeqCst);
        loop {
            arch::wait_interrupt();
            let t = PIT_CLOCK.load(Ordering::SeqCst);
            if t != 0 {
                return t;
            }
        }
    }

    pub fn pit_sleep(time: Duration) {
        let millis = time.as_millis() as usize;
        PIT_CLOCK.store(0, Ordering::SeqCst);
        while PIT_CLOCK.load(Ordering::SeqCst) <= millis {
            arch::wait_interrupt()
        }
    } */
}

impl Pic {
    fn cmd(&mut self, cmd: u8) {
        unsafe { self.cmd.write(cmd) };
        iowait();
    }

    fn data(&mut self, v: u8) {
        unsafe { self.data.write(v) };
        iowait();
    }

    fn read_mask(&mut self) -> u8 {
        unsafe { self.data.read() }
    }

    fn write_mask(&mut self, v: u8) {
        unsafe { self.data.write(v) }
    }

    fn has(&self, irq: u8) -> bool {
        (self.offs..self.offs + 8).contains(&irq)
    }
}

//

fn iowait() {
    unsafe { Port::<u8>::new(0x80).write(0) }
}
