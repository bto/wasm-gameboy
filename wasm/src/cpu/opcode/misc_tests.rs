use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_ccf() {
    let mut cpu = CPU::new();
    let opcode = 0x3F;

    // true -> false
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.carry = true;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, true);

    // false -> true
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.carry = false;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);
}

#[test]
fn op_cpl_r() {
    let mut cpu = CPU::new();
    let opcode = 0x2F;

    // 0b10010110
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.a = 0b10010110;
    cpu.registers.carry = true;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0b01101001);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);

    // 0b01101001
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.a = 0b01101001;
    cpu.registers.carry = false;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.a, 0b10010110);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, true);
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, false);
}

#[test]
fn op_daa_r() {
    let mut cpu = CPU::new();
    let opcode = 0x27;

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
    assert_eq!(cpu.registers.subtraction, false);
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
    assert_eq!(cpu.registers.subtraction, false);
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
    assert_eq!(cpu.registers.subtraction, true);
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
    assert_eq!(cpu.registers.subtraction, true);
    assert_eq!(cpu.registers.zero, true);
}

#[test]
fn op_halt() {
    let mut cpu = CPU::new();
    let opcode = 0x76;

    // true -> true
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.halt = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.halt, true);

    // false -> true
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.halt = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.halt, true);
}

#[test]
fn op_scf() {
    let mut cpu = CPU::new();
    let opcode = 0x37;

    // true -> true
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.carry = true;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, true);

    // false -> true
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.registers.carry = false;
    cpu.registers.half_carry = false;
    cpu.registers.subtraction = false;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.registers.carry, true);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);
}

#[test]
fn op_stop() {
    let mut cpu = CPU::new();
    let opcode = 0x10;

    // true -> true
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.stop = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.stop, true);

    // false -> true
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, opcode);
    cpu.stop = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 1);
    assert_eq!(cpu.stop, true);
}

#[test]
fn op_swap_r() {
    let mut cpu = CPU::new();

    let opcode_base = 0b00110_000;
    for i in [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b111].iter() {
        let opcode = opcode_base | i;

        // SWAP 0b1010_0101
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, 0xCB, opcode);
        match i {
            0b000 => cpu.registers.b = 0b1010_0101,
            0b001 => cpu.registers.c = 0b1010_0101,
            0b010 => cpu.registers.d = 0b1010_0101,
            0b011 => cpu.registers.e = 0b1010_0101,
            0b100 => cpu.registers.h = 0b1010_0101,
            0b101 => cpu.registers.l = 0b1010_0101,
            0b111 => cpu.registers.a = 0b1010_0101,
            _ => panic!("never reach"),
        }
        cpu.registers.carry = true;
        cpu.registers.half_carry = true;
        cpu.registers.subtraction = true;
        cpu.registers.zero = true;
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 2);
        assert_eq!(cpu.registers.carry, false);
        assert_eq!(cpu.registers.half_carry, false);
        assert_eq!(cpu.registers.subtraction, false);
        assert_eq!(cpu.registers.zero, false);
        match i {
            0b000 => assert_eq!(cpu.registers.b, 0b0101_1010),
            0b001 => assert_eq!(cpu.registers.c, 0b0101_1010),
            0b010 => assert_eq!(cpu.registers.d, 0b0101_1010),
            0b011 => assert_eq!(cpu.registers.e, 0b0101_1010),
            0b100 => assert_eq!(cpu.registers.h, 0b0101_1010),
            0b101 => assert_eq!(cpu.registers.l, 0b0101_1010),
            0b111 => assert_eq!(cpu.registers.a, 0b0101_1010),
            _ => panic!("never reach"),
        }

        // SWAP 0
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, 0xCB, opcode);
        match i {
            0b000 => cpu.registers.b = 0,
            0b001 => cpu.registers.c = 0,
            0b010 => cpu.registers.d = 0,
            0b011 => cpu.registers.e = 0,
            0b100 => cpu.registers.h = 0,
            0b101 => cpu.registers.l = 0,
            0b111 => cpu.registers.a = 0,
            _ => panic!("never reach"),
        }
        cpu.registers.carry = true;
        cpu.registers.half_carry = true;
        cpu.registers.subtraction = true;
        cpu.registers.zero = false;
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 2);
        assert_eq!(cpu.registers.carry, false);
        assert_eq!(cpu.registers.half_carry, false);
        assert_eq!(cpu.registers.subtraction, false);
        assert_eq!(cpu.registers.zero, true);
        match i {
            0b000 => assert_eq!(cpu.registers.b, 0),
            0b001 => assert_eq!(cpu.registers.c, 0),
            0b010 => assert_eq!(cpu.registers.d, 0),
            0b011 => assert_eq!(cpu.registers.e, 0),
            0b100 => assert_eq!(cpu.registers.h, 0),
            0b101 => assert_eq!(cpu.registers.l, 0),
            0b111 => assert_eq!(cpu.registers.a, 0),
            _ => panic!("never reach"),
        }
    }
}

#[test]
fn op_swap_rrn() {
    let mut cpu = CPU::new();
    let opcode = 0b00110_110;

    // SWAP 0b1010_0101
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0xCB, opcode);
    cpu.registers.h = 3;
    cpu.registers.l = 2;
    cpu.mmu.byte_set(0x302, 0b1010_0101);
    cpu.registers.carry = true;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = true;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.mmu.byte_get(0x302), 0b0101_1010);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, false);

    // SWAP 0
    let pc = cpu.registers.pc;
    set_inst!(cpu, pc, 0xCB, opcode);
    cpu.registers.h = 4;
    cpu.registers.l = 3;
    cpu.mmu.byte_set(0x403, 0);
    cpu.registers.carry = true;
    cpu.registers.half_carry = true;
    cpu.registers.subtraction = true;
    cpu.registers.zero = false;
    cpu.execute();
    assert_eq!(cpu.registers.pc, pc + 2);
    assert_eq!(cpu.mmu.byte_get(0x403), 0);
    assert_eq!(cpu.registers.carry, false);
    assert_eq!(cpu.registers.half_carry, false);
    assert_eq!(cpu.registers.subtraction, false);
    assert_eq!(cpu.registers.zero, true);
}
