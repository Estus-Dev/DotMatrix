use dotmatrix::DotMatrix;

#[test]
fn blargg_cpu_01_special() {
    let mut dmg = DotMatrix::new_dmg();
    let rom = include_bytes!("../../test_data/blargg/cpu_instrs/individual/01-special.gb");

    dmg.load(rom.as_slice().into());

    loop {
        dmg.exec_instruction();

        // This is the address of the final instruction of the test ROM.
        // I expect to replace this with a run condition system.
        let self_loop_addr: u16 = 0xFE_18;
        if self_loop_addr == dmg.bus.read16(dmg.cpu.pc) {
            break;
        }
    }
}
