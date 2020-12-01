use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_push_rr() {
    let mut cpu = CPU::new();

    cpu.registers.bc_set(0x102);
    cpu.registers.de_set(0x304);
    cpu.registers.hl_set(0x506);
    cpu.registers.af_set(0x7FF);
    cpu.registers.sp = 0x908;

    let opcode_base = 0b11_00_0101;
    for i in [0b00, 0b01, 0b10, 0b11].iter() {
        let pc = cpu.registers.pc;
        let opcode = opcode_base | i << 4;
        set_inst!(cpu, pc, opcode);

        cpu.execute();

        assert_eq!(cpu.registers.pc, pc + 1);
    }

    assert_eq!(cpu.mmu.byte_get(0x900), 0xF0);
    assert_eq!(cpu.mmu.byte_get(0x901), 7);
    assert_eq!(cpu.mmu.byte_get(0x902), 6);
    assert_eq!(cpu.mmu.byte_get(0x903), 5);
    assert_eq!(cpu.mmu.byte_get(0x904), 4);
    assert_eq!(cpu.mmu.byte_get(0x905), 3);
    assert_eq!(cpu.mmu.byte_get(0x906), 2);
    assert_eq!(cpu.mmu.byte_get(0x907), 1);
}

#[test]
fn op_pop_rr() {
    let mut cpu = CPU::new();

    cpu.mmu.word_set(0x200, 0x102);
    cpu.mmu.word_set(0x202, 0x304);
    cpu.mmu.word_set(0x204, 0x506);
    cpu.mmu.word_set(0x206, 0x7FF);
    cpu.registers.sp = 0x200;

    let opcode_base = 0b11_00_0001;
    for i in [0b00, 0b01, 0b10, 0b11].iter() {
        let pc = cpu.registers.pc;
        let opcode = opcode_base | i << 4;
        set_inst!(cpu, pc, opcode);

        cpu.execute();

        assert_eq!(cpu.registers.pc, pc + 1);
    }

    assert_eq!(cpu.registers.b, 1);
    assert_eq!(cpu.registers.c, 2);
}
