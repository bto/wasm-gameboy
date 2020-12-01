use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_daa() {
    let mut cpu = CPU::new();
    let opcode = 0b00100111;

    // 0x12 + 0x34
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b11_000_110, 0x34);
    cpu.registers.a = 0x12;
    cpu.execute();

    // DAA
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0x46);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.zero, false);

    // 0x55 + 0x45
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b11_000_110, 0x45);
    cpu.registers.a = 0x55;
    cpu.execute();

    // DAA
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.zero, true);

    // 0x55 - 0x58
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b11_010_110, 0x58);
    cpu.registers.a = 0x55;
    cpu.execute();

    // DAA
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0x97);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.zero, false);

    // 0x55 - 0x55
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0b11_010_110, 0x55);
    cpu.registers.a = 0x55;
    cpu.execute();

    // DAA
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.zero, true);
}
