use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_ld_rr_nn() {
    let mut cpu = CPU::new();

    // LD rr, nn
    let opcode_base = 0b00_00_0001;
    for i in [0b00, 0b01, 0b10, 0b11].iter() {
        let pc = cpu.registers.pc;
        let opcode = opcode_base | i << 4;
        let lsb = *i;
        let msb = lsb + 1;
        set_inst!(cpu, pc, opcode, lsb, msb);

        cpu.execute();

        assert_eq!(cpu.registers.pc, pc + 3);
        let v = (msb as u16) << 8 | lsb as u16;
        match i {
            0b00 => assert_eq!(cpu.registers.bc_get(), v),
            0b01 => assert_eq!(cpu.registers.de_get(), v),
            0b10 => assert_eq!(cpu.registers.hl_get(), v),
            0b11 => assert_eq!(cpu.registers.sp, v),
            _ => panic!("never reach"),
        }
    }
}

#[test]
fn op_ld_rr_rr() {
    let mut cpu = CPU::new();

    // LD SP, HL
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0xF9);

    cpu.registers.h = 1;
    cpu.registers.l = 2;

    cpu.execute();

    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.sp, 0x102);
}
