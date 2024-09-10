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
