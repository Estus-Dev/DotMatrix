/// A cartridge plugged into the system, with its own bus pointing to ROM, optional RAM, and other
/// MMIO like a camera, accelerometer, or real time clock.
pub struct Cartridge {
    rom: Box<[u8]>,
}

impl Cartridge {
    pub fn new(data: Box<[u8]>) -> Self {
        Self { rom: data }
    }

    /// Read an 8-bit value from the specified address. Affected by cartridge state.
    pub fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
}
