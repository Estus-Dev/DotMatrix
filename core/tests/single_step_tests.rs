//! The Single Step Tests involve setting up your emulator to match a given state, running a single
//! instruction, and then comparing to expected state.

use std::{fmt::Debug, fs, path::Path};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use dotmatrix::DotMatrix;

/// Generate separate test fns for each SingleStepTest in the JSON data, so that each test result
/// shows up separately when running tests via cargo.
///
/// This is simpler than a custom test runner, though a runner may be preferable in the long term.
macro_rules! single_step_test_opcodes {
    ($($opcode:literal),+) => {
        $(
            paste::paste! {
                #[test]
                fn [<single_step_opcode_ $opcode>]() {
                    test_opcode(&$opcode.replace('_', " "));
                }
            }
        )+
    };
}

// Invoke the above macro with the implemented opcodes.
single_step_test_opcodes! {
    "00"
}

/// The actual meat of the tests. Iterates through a SingleStepTest JSON file and for each test case
/// compares the expected state to the actual state.
///
/// On failure this will dump the full initial, expected, and actual state of the first failed case
/// to the console for ease of debugging. Dumping only the part or parts that mismatch makes it more
/// difficult to reason about than just showing the full before and after.
fn test_opcode(opcode: &str) {
    let cases = load_test(opcode);

    for case in cases {
        let mut dmg: DotMatrix = case.initial_state.clone().into();

        dmg.exec_instruction();

        let addrs: Vec<u16> = case.final_state.ram.iter().map(|(addr, _)| *addr).collect();
        let dmg_state = State::new(&dmg, &addrs);

        assert!(
            case.final_state == dmg_state,
            "Opcode {}\n  initial: {:?}\n  expected: {:?}\n  result: {:?}",
            &case.name,
            &case.initial_state,
            &case.final_state,
            &dmg_state,
        );
    }
}

/// Load a test file from disk matching the given opcode.
fn load_test(opcode: &str) -> Vec<SM83TestCase> {
    let path = format!(
        "../test_data/single_step_tests/v1/{}.json",
        opcode.to_lowercase()
    );
    let json = fs::read(Path::new(&path)).unwrap_or_else(|_| {
        panic!("Could not load \"{path}\", try running `git submodule update --init`")
    });

    serde_json::from_slice(&json).unwrap()
}

/// Represents a single test from the SingleStepTests/sm83 test data.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
struct SM83TestCase {
    /// The name of the test. First the opcode, then the test number.
    name: String,

    /// The state the system should be initialized to before executing the test.
    #[serde(rename = "initial")]
    initial_state: State,

    /// The state the system should be in after executing the test.
    #[serde(rename = "final")]
    final_state: State,

    // TODO: Handle cycles
    /// A list of all cycles
    cycles: Vec<(u16, u8, String)>,
}

/// The state of the system, before or after a test.
#[serde_as]
#[derive(Clone, Default, Deserialize, Eq, PartialEq, Serialize)]
struct State {
    /// The status of the `PC` register.
    pc: u16,

    /// The status of the `SP` register.
    sp: u16,

    /// The status of the `A` register.
    a: u8,

    /// The status of the `B` register.
    b: u8,

    /// The status of the `C` register.
    c: u8,

    /// The status of the `D` register.
    d: u8,

    /// The status of the `E` register.
    e: u8,

    /// The status of the `F` register.
    f: u8,

    /// The status of the `H` register.
    h: u8,

    /// The status of the `L` register.
    l: u8,

    // TODO: Handle IME
    /// The status of the `IME` register.
    // #[serde_as(as = "BoolFromInt")]
    // ime: bool,

    // TODO: Handle IE
    /// The status of the `IME` register, usually only on `initial` state.
    // ie: Option<u8>,

    /// A tuple of memory addresses to values in that address.
    ram: Vec<(u16, u8)>,
}

impl State {
    /// Pull out comparable state from an instance of DotMatrix.
    ///
    /// The only reason this isn't `From<DotMatrix>` is because we only want to compare specific
    /// addresses.
    fn new(dmg: &DotMatrix, ram_addrs: &[u16]) -> Self {
        Self {
            pc: dmg.cpu.pc,
            sp: dmg.cpu.sp,
            a: dmg.cpu.registers.a(),
            b: dmg.cpu.registers.b(),
            c: dmg.cpu.registers.c(),
            d: dmg.cpu.registers.d(),
            e: dmg.cpu.registers.e(),
            f: dmg.cpu.registers.f(),
            h: dmg.cpu.registers.h(),
            l: dmg.cpu.registers.l(),
            ram: ram_addrs
                .iter()
                .map(|&addr| (addr, dmg.bus.read(addr)))
                .collect(),
        }
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let z = (self.f & (1 << 7)) >> 7;
        let n = (self.f & (1 << 6)) >> 6;
        let h = (self.f & (1 << 5)) >> 5;
        let c = (self.f & (1 << 4)) >> 4;

        writeln!(f, "State {{")?;
        write!(f, "\tCPU {{ ")?;

        write!(f, "A:{:02X} ", self.a)?;

        write!(f, "c:{:01} ", c)?;
        write!(f, "h:{:01} ", h)?;
        write!(f, "n:{:01} ", n)?;
        write!(f, "z:{:01} ", z)?;

        write!(f, "BC:{:04X} ", u16::from_le_bytes([self.c, self.b]))?;
        write!(f, "DE:{:04X} ", u16::from_le_bytes([self.e, self.d]))?;
        write!(f, "HL:{:04X} ", u16::from_le_bytes([self.l, self.h]))?;

        write!(f, "SP:{:04X} ", self.sp)?;
        write!(f, "PC:{:04X} ", self.pc)?;

        writeln!(f, "}}")?;
        write!(f, "\tRAM {{ ")?;

        for (addr, value) in &self.ram {
            write!(f, "{addr:04X}:{value:02X} ")?;
        }

        writeln!(f, "}}")?;
        write!(f, "}}")
    }
}

impl From<State> for DotMatrix {
    fn from(state: State) -> Self {
        let mut dmg = DotMatrix::new_with_flat_bus();

        dmg.cpu.registers.set_a(state.a);
        dmg.cpu.registers.set_b(state.b);
        dmg.cpu.registers.set_c(state.c);
        dmg.cpu.registers.set_d(state.d);
        dmg.cpu.registers.set_e(state.e);
        dmg.cpu.registers.set_f(state.f);
        dmg.cpu.registers.set_h(state.h);
        dmg.cpu.registers.set_l(state.l);

        dmg.cpu.pc = state.pc;
        dmg.cpu.sp = state.sp;

        for (address, value) in state.ram {
            dmg.bus.write(address, value);
        }

        dmg
    }
}
