mod bus;
mod cpu;

pub use bus::Bus;
use cpu::Sm83;

pub struct DotMatrix {
    pub bus: Bus,
    pub cpu: Sm83,
}

impl DotMatrix {
    /// Create a new [DotMatrix] DMG.
    pub fn new_dmg() -> DotMatrix {
        Self {
            bus: Bus::new(),
            cpu: Sm83::new_dmg(),
        }
    }

    /// Create a new [DotMatrix] DMG with a flat [Bus] for testing purposes.
    #[cfg(test)]
    pub fn new_with_flat_bus() -> DotMatrix {
        Self {
            bus: Bus::flat(),
            cpu: Sm83::new_dmg(),
        }
    }
}
