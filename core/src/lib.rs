mod bus;
mod cartridge;
mod cpu;

use std::rc::Rc;

pub use bus::Bus;
use cartridge::Cartridge;
use cpu::Sm83;

pub struct DotMatrix {
    pub bus: Bus,
    pub cpu: Sm83,
    pub cartridge: Option<Rc<Cartridge>>,
}

impl DotMatrix {
    /// Create a new [DotMatrix] DMG.
    pub fn new_dmg() -> DotMatrix {
        Self {
            bus: Bus::new_dmg(),
            cpu: Sm83::new_dmg(),
            cartridge: None,
        }
    }

    /// Create a new [DotMatrix] DMG with a flat [Bus] for testing purposes.
    pub fn new_with_flat_bus() -> DotMatrix {
        Self {
            bus: Bus::flat(),
            cpu: Sm83::new_dmg(),
            cartridge: None,
        }
    }

    pub fn load(&mut self, rom: Box<[u8]>) {
        self.cartridge = Some(Rc::new(Cartridge::new(rom)));
    }

    /// Execute until the end of the current CPU instruction. Fetches if queue is empty.
    ///
    /// For testing purposes, specifically SingleStepTests.
    pub fn exec_instruction(&mut self) {
        self.cpu.exec_instruction(&mut self.bus);
    }
}
