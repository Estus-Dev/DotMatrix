use std::str::FromStr;

use crate::Bus;

use super::Sm83;

/// Break each instruction on the SM83 down to the actions to perform each machine cycle (m-cycle).
/// I'm calling this m-code, and I'm not basing it directly on any microcode the SM83 may or may not
/// have.
///
/// These are not based directly on any SM83 microcode, but are instead pulled from diagrams in the
/// [Gameboy Complete Technical Reference](https://github.com/Gekkio/gb-ctr) by
/// [Gekkio](https://github.com/Gekkio).
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum MCode {
    /// Perform no action.
    Nop,

    /// An illegal instruction, halts execution immediately.
    Illegal,
}

impl MCode {
    pub fn exec(&self, _cpu: &mut Sm83, _bus: &mut Bus) {
        match self {
            Self::Nop => (),
            Self::Illegal => panic!("Illegal instruction encountered"),
        }
    }
}

impl FromStr for MCode {
    type Err = ();
    fn from_str(mcode: &str) -> Result<Self, Self::Err> {
        use MCode as M;

        Ok(match mcode {
            "NOP" => M::Nop,
            "ILLEGAL" => M::Illegal,
            _ => return Err(()),
        })
    }
}
