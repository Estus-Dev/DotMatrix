const ADDRESS_SPACE: usize = 0x1_0000;
const PAGE_SIZE: usize = 0x100;
const PAGE_COUNT: usize = ADDRESS_SPACE / PAGE_SIZE;

/// A 256-item chunk of address space, indexed by a `u8`. Can be wired to RAM, ROM, or specialized
/// hardware.
enum Page {
    /// Readable and writable memory.
    Ram([u8; PAGE_SIZE]),
}

impl Page {
    fn read(&self, addr: u8) -> u8 {
        match self {
            Self::Ram(ram) => ram[addr as usize],
        }
    }

    fn write(&mut self, addr: u8, value: u8) {
        match self {
            Self::Ram(ram) => ram[addr as usize] = value,
        }
    }
}

impl Page {
    const fn new_ram() -> Self {
        Self::Ram([0xFF; PAGE_SIZE])
    }
}

/// The main bus of the system. Divided into [Pages](Page) based on the [Memory Map][].
/// Addresses are 16 bits wide and values are 8 bits wide.
///
/// [Memory Map]: https://gbdev.io/pandocs/Memory_Map.html
pub struct Bus([Page; PAGE_COUNT]);

impl Bus {
    /// Read a value from the specified address.
    pub fn read(&self, addr: u16) -> u8 {
        let [index, page] = addr.to_le_bytes();

        self.0[page as usize].read(index)
    }

    /// Write a value to the specified address.
    pub fn write(&mut self, addr: u16, value: u8) {
        let [index, page] = addr.to_le_bytes();

        self.0[page as usize].write(index, value);
    }
}

impl Bus {
    /// Create a new [Bus] with the standard memory map for the DMG.
    pub fn new_dmg() -> Self {
        // TODO: Proper memory map
        const RAM: Page = Page::new_ram();

        Self([RAM; PAGE_COUNT])
    }

    /// Create a new [Bus] with nothing but RAM for use with the [Single Step Tests][].
    ///
    /// [Single Step Tests]: https://github.com/SingleStepTests/sm83
    pub fn flat() -> Self {
        const RAM: Page = Page::new_ram();

        Self([RAM; PAGE_COUNT])
    }
}
