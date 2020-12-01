use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_add_rr_rr() {
    let mut cpu = CPU::new();

    let opcode_base = 0b00_00_1001;
    for i in [0b00, 0b01, 0b10, 0b11].iter() {
        let opcode = opcode_base | (i << 4);

        // 0x8000 + 0x8000
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, opcode);
        cpu.registers.hl_set(0x8000);
        match i {
            0b00 => cpu.registers.bc_set(0x8000),
            0b01 => cpu.registers.de_set(0x8000),
            0b10 => cpu.registers.hl_set(0x8000),
            0b11 => cpu.registers.sp = 0x8000,
            _ => panic!("never reach"),
        }
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 1);
        assert_eq!(cpu.registers.hl_get(), 0);
        assert_eq!(cpu.registers.carry, true);
        assert_eq!(cpu.registers.half_carry, false);
        assert_eq!(cpu.registers.subtraction, false);
        assert_eq!(cpu.registers.zero, true);

        // 0b00000100_00000000 + 0b00000100_00000000
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, opcode);
        cpu.registers.hl_set(0b00000100_00000000);
        match i {
            0b00 => cpu.registers.bc_set(0b00000100_00000000),
            0b01 => cpu.registers.de_set(0b00000100_00000000),
            0b10 => cpu.registers.hl_set(0b00000100_00000000),
            0b11 => cpu.registers.sp = 0b00000100_00000000,
            _ => panic!("never reach"),
        }
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 1);
        assert_eq!(cpu.registers.hl_get(), 0b00001000_00000000);
        assert_eq!(cpu.registers.carry, false);
        assert_eq!(cpu.registers.half_carry, true);
        assert_eq!(cpu.registers.subtraction, false);
        assert_eq!(cpu.registers.zero, false);
    }
}
