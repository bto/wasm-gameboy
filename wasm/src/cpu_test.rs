use super::*;

macro_rules! set_inst {
    ( $cpu:ident, $pc:ident, $v1:expr, $v2:expr ) => {
        $cpu.mmu.byte_set($pc + 0, $v1);
        $cpu.mmu.byte_set($pc + 1, $v2);
    };
}

#[test]
fn test_new() {
    let cpu = CPU::new();
    assert_eq!(cpu.mmu, MMU::new());
    assert_eq!(cpu.registers, Registers::new());
}

#[test]
fn test_op_ld_r_n() {
    let mut cpu = CPU::new();

    // LD B, n
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0x06, 1);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.b, 1);

    // LD C, n
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0x0E, 2);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.c, 2);

    // LD D, n
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0x16, 3);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.d, 3);

    // LD E, n
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0x1E, 4);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.e, 4);

    // LD H, n
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0x26, 5);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.h, 5);

    // LD L, n
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0x2E, 6);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.registers.l, 6);
}
