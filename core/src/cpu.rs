use std::{collections::VecDeque, fmt::Debug};

use dotmatrix_opcodes::MCode;
use dotmatrix_opcodes::Opcode;
use proc_bitfield::bitfield;

use crate::Bus;

/// The value of PC _after running the boot ROM_.
const AFTER_BOOT_PC: u16 = 0x0100;

/// The value of SP _after running the boot ROM_.
const AFTER_BOOT_SP: u16 = 0xFFFE;

/// The SM83 by Sharp is the CPU used in the DMG. It is distinct from a Zilog Z80 despite several
/// similarities.
///
/// Names are not short for the purpose of saving characters, these are the names the community
/// and documentation have settled upon.
#[derive(Clone, Eq, PartialEq)]
pub struct Sm83 {
    /// The general purpose registers and flags of the [Sm83].
    pub registers: Sm83Registers,

    /// The program counter, points to the next instruction in memory.
    pub pc: u16,

    /// The stack pointer, points to the "top" stack frame in memory. _(The stack grows downward)_
    pub sp: u16,

    /// The instruction register holds the opcode of the currently executing instruction.
    pub ir: Opcode,

    /// A queue of m-codes to be executed over the next few cycles.
    pub mcode_queue: VecDeque<MCode>,
}

impl Sm83 {
    /// Create a new [Sm83] configured for use in a DMG.
    pub fn new_dmg() -> Self {
        Self {
            registers: Sm83Registers::initial_dmg(),
            pc: AFTER_BOOT_PC,
            sp: AFTER_BOOT_SP,
            ir: Opcode::NOP,
            mcode_queue: VecDeque::with_capacity(8),
        }
    }

    /// Execute one m-cycle worth of code on the CPU.
    pub fn exec_m_cycle(&mut self, bus: &mut Bus) {
        // Fetching the next instruction and executing the current overlap by one m-cycle.
        if self.mcode_queue.len() <= 1 {
            self.fetch(bus);
        }

        let mcode = self
            .mcode_queue
            .pop_front()
            .expect("Attempted to pop from empty mcode_queue");

        self.exec_mcode(mcode, bus);
    }

    /// Execute until the end of the current instruction. Fetches an instruction if queue is empty.
    ///
    /// For testing purposes, specifically SingleStepTests.
    pub fn exec_instruction(&mut self, bus: &mut Bus) {
        if self.mcode_queue.is_empty() {
            self.fetch(bus);
        }

        while let Some(mcode) = self.mcode_queue.pop_front() {
            self.exec_mcode(mcode, bus);
        }
    }

    /// Retrieve the next instruction and increment PC.
    pub fn fetch(&mut self, bus: &mut Bus) {
        self.ir = bus.read(self.pc).into();
        self.ir
            .mcode()
            .iter()
            .for_each(|&mcode| self.mcode_queue.push_back(mcode));

        self.pc += 1;
    }

    fn exec_mcode(&mut self, mcode: MCode, _bus: &mut Bus) {
        match mcode {
            MCode::Nop => (),
            MCode::Illegal => panic!(
                "Illegal instruction encountered: {:#04X} ({})",
                self.ir as u8, self.ir
            ),
        }
    }
}

impl Debug for Sm83 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sm83 {{ ")?;

        write!(f, "A:{:02X} ", self.registers.a())?;

        write!(f, "c:{:01} ", self.registers.c_flag() as usize)?;
        write!(f, "h:{:01} ", self.registers.h_flag() as usize)?;
        write!(f, "n:{:01} ", self.registers.n_flag() as usize)?;
        write!(f, "z:{:01} ", self.registers.z_flag() as usize)?;

        write!(f, "BC:{:04X} ", self.registers.bc())?;
        write!(f, "DE:{:04X} ", self.registers.de())?;
        write!(f, "HL:{:04X} ", self.registers.hl())?;

        write!(f, "SP:{:04X} ", self.sp)?;
        write!(f, "PC:{:04X} ", self.pc)?;

        write!(f, "}}")
    }
}

bitfield! {
    /// The general purpose 8 and 16 bit registers of the SM83, including the flags.
    ///
    /// Names are not short for the purpose of saving characters, these are the names the community
    /// and documentation have settled upon.
    #[derive(Clone, Copy, Eq, PartialEq)]
    pub struct Sm83Registers(u64): Debug, FromRaw, IntoRaw, DerefRaw {
        /// The `c` flag (carry flag) is set when a carry or borrow occurs in an arithmetic
        /// operation. It is also the 4th bit of the virtual `F` register in AF.
        pub c_flag: bool @ 4,

        /// The `h` flag (half-carry flag) is set whenever a carry would occur 8 bits below the most
        /// significant bit. It's used by BCD operations. It is also the 5th bit of the virtual `F`
        /// register in AF.
        pub h_flag: bool @ 5,

        /// The `n` flag (subtraction flag) is set whenever a subtraction occurs, to assist in BCD
        /// operations. It is also the 6th bit of the virtual `F` register in AF.
        pub n_flag: bool @ 6,

        /// The `z` flag (zero flag) is set when a calculation results in a value of `0`.
        /// It is also the 7th bit of the virtual `F` register in AF.
        pub z_flag: bool @ 7,

        /// The virtual "`F`" register is comprised of flags, in the form z/n/h/c/0/0/0/0. It is
        /// used as the low bits of AF.
        ///
        /// This virtual register is not accessed by the hardware except in the combined case of
        /// `AF`. It's provided here for the sake of testing, deugging, visualization, or logging.
        pub f: u8 [get_fn(|f| f & 0xF0)] @ 0..=7,

        /// The `A` register is the accumulator, and is used as the high bits of AF.
        pub a: u8 @ 8..=15,

        /// The `AF` register is the A register and the flags combined. Low 4 bits are always `0`.
        /// This is the only way the hardware accesses the virtual "`F`" register.
        pub af: u16 [get_fn(|af| af & 0xFFF0)] @ 0..=15,

        /// The `C` register is a general-purpose register and the low bits of BC.
        pub c: u8 @ 16..=23,

        /// The `B` register is a general-purpose register and the high bits of BC.
        pub b: u8 @ 24..=31,

        /// The `BC` register is the B and C registers combined.
        pub bc: u16 @ 16..=31,

        /// The `E` register is a general-purpose register and the low bits of DE.
        pub e: u8 @ 32..=39,

        /// The `D` register is a general-purpose register and the high bits of DE.
        pub d: u8 @ 40..=47,

        /// The `DE` register is the D and E registers combined.
        pub de: u16 @ 32..=47,

        /// The `L` register is a general-purpose register and the low bits of HL.
        pub l: u8 @ 48..=55,

        /// The `H` register is a general-purpose register and the high bits of HL.
        pub h: u8 @ 56..=63,

        /// The `HL` register is the H and L registers combined. It's often used to hold a pointer,
        /// and can be incremented/decremented on access by some operations.
        pub hl: u16 @ 48..=63,
    }
}

impl Sm83Registers {
    /// The initial state of registers on DMG, via the Cycle Accurate GB Docs.
    pub fn initial_dmg() -> Self {
        //   0xHH_LL_DD_EE_BB_CC_AA_FF
        Self(0x01_4D_00_D8_00_13_01_B0)
    }

    /// The initial state of registers on MGB, via the Cycle Accurate GB Docs.
    pub fn initial_mgb() -> Self {
        //   0xHH_LL_DD_EE_BB_CC_AA_FF
        Self(0x01_4D_00_D8_00_13_FF_B0)
    }

    /// The initial state of registers on SGB, via the Cycle Accurate GB Docs.
    ///
    /// Note: TCAGBD states these have not been verified on hardware.
    pub fn initial_sgb() -> Self {
        //   0xHH_LL_DD_EE_BB_CC_AA_FF
        Self(0xC0_60_00_00_00_14_01_00)
    }

    /// The initial state of registers on SGB2, via the Cycle Accurate GB Docs.
    ///
    /// Note: TCAGBD does not specify anything but the value of `A`, so I'm defaulting them to
    /// [Registers::initial_sgb] for now. See note there.
    pub fn initial_sgb2() -> Self {
        let mut registers = Self::initial_sgb();

        registers.set_a(0xFF);

        registers
    }

    /// The initial state of registers on CGB, via the Cycle Accurate GB Docs.
    pub fn initial_cgb() -> Self {
        //   0xHH_LL_DD_EE_BB_CC_AA_FF
        Self(0x00_7C_00_08_00_00_11_80)
    }

    /// The initial state of registers on AGB, via the Cycle Accurate GB Docs.
    pub fn initial_agb() -> Self {
        //   0xHH_LL_DD_EE_BB_CC_AA_FF
        Self(0x00_7C_00_08_01_00_11_00)
    }

    /// The initial state of registers on AGS, via the Cycle Accurate GB Docs.
    pub fn initial_ags() -> Self {
        //   0xHH_LL_DD_EE_BB_CC_AA_FF
        Self(0x00_7C_00_08_01_00_11_00)
    }
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn sm83_debug() {
        let expected = "Sm83 { A:CD c:1 h:0 n:1 z:0 BC:89AB DE:4567 HL:0123 SP:A801 PC:532D }";
        let registers = Sm83Registers(0x01_23_45_67_89_AB_CD_50);
        let cpu = Sm83 {
            registers,
            pc: 0x532D,
            sp: 0xA801,
            ir: Opcode::NOP,
            mcode_queue: VecDeque::with_capacity(0),
        };

        assert_eq!(expected, &format!("{cpu:?}"));
    }
}
