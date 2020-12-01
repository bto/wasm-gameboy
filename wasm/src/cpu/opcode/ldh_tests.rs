use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_ldh_rh_r() {
    let mut cpu = CPU::new();

    // LDH (C), A
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b11100010);

    cpu.registers.a = 1;
    cpu.registers.c = 2;

    cpu.execute();

    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.mmu.byte_get(0xFF02), 1);
}

#[test]
fn op_ldh_r_rh() {
    let mut cpu = CPU::new();

    // LDH A, (C)
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b11110010);

    cpu.registers.c = 1;
    cpu.mmu.byte_set(0xFF01, 2);

    cpu.execute();

    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 2);
}

#[test]
fn op_ldh_nh_r() {
    let mut cpu = CPU::new();

    // LDH (n), A
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b11100000, 1);

    cpu.registers.a = 2;

    cpu.execute();

    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.mmu.byte_get(0xFF01), 2);
}

#[test]
fn op_ldh_r_nh() {
    let mut cpu = CPU::new();

    // LDH A, (n)
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b11110000, 1);

    cpu.mmu.byte_set(0xFF01, 2);

    cpu.execute();

    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.a, 2);
}
