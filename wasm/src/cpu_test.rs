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

    let opcode_base = 0b00_000_110;
    for i in [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b111].iter() {
        let pc = cpu.registers.pc;
        let opcode = opcode_base | (i << 3);
        set_inst!(cpu, pc, opcode, *i);
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 2);
        match i {
            0b000 => assert_eq!(cpu.registers.b, *i),
            0b001 => assert_eq!(cpu.registers.c, *i),
            0b010 => assert_eq!(cpu.registers.d, *i),
            0b011 => assert_eq!(cpu.registers.e, *i),
            0b100 => assert_eq!(cpu.registers.h, *i),
            0b101 => assert_eq!(cpu.registers.l, *i),
            0b111 => assert_eq!(cpu.registers.a, *i),
            _ => panic!("never reach"),
        }
    }
}
