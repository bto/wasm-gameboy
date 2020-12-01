use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_ld_hln_r() {
    // LD (HL), r
    let mut cpu = CPU::new();

    let opcode_base = 0b01_110_000;
    for i in [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b111].iter() {
        let pc = cpu.registers.pc;
        let opcode = opcode_base | i;
        set_inst!(cpu, pc, opcode);

        cpu.registers.h = 2;
        cpu.registers.l = 0;

        let v = *i;
        match i {
            0b000 => cpu.registers.b = v,
            0b001 => cpu.registers.c = v,
            0b010 => cpu.registers.d = v,
            0b011 => cpu.registers.e = v,
            0b100 => cpu.registers.h = v,
            0b101 => cpu.registers.l = v,
            0b111 => cpu.registers.a = v,
            _ => panic!("never reach"),
        }

        cpu.execute();

        assert_eq!(cpu.registers.pc, pc + 1);
        assert_eq!(cpu.mmu.byte_get(cpu.registers.hl_get()), v);
    }
}

#[test]
fn op_ld_rrn_n() {
    // LD (HL), n
    let mut cpu = CPU::new();

    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00_110_110, 1);

    cpu.registers.h = 2;
    cpu.registers.l = 0;

    cpu.execute();

    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.mmu.byte_get(0x200), 1);
}

#[test]
fn op_ld_rrn_r() {
    let mut cpu = CPU::new();

    // LD (BC), A
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00_00_0010);
    cpu.registers.b = 2;
    cpu.registers.c = 1;
    cpu.registers.a = 3;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.mmu.byte_get(0x201), 3);

    // LD (DE), A
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b00_01_0010);
    cpu.registers.d = 3;
    cpu.registers.e = 2;
    cpu.registers.a = 4;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.mmu.byte_get(0x302), 4);
}
