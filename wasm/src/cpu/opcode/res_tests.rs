use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_res_r() {
    let mut cpu = CPU::new();

    let opcode_base = 0b10_000_000;
    for b in [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111].iter() {
        for i in [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b111].iter() {
            let opcode = opcode_base | (b << 3) | i;

            // 0b0000_0000
            let pc = cpu.registers.pc;
            set_inst!(cpu, pc, 0xCB, opcode);
            match i {
                0b000 => cpu.registers.b = 0b0000_0000,
                0b001 => cpu.registers.c = 0b0000_0000,
                0b010 => cpu.registers.d = 0b0000_0000,
                0b011 => cpu.registers.e = 0b0000_0000,
                0b100 => cpu.registers.h = 0b0000_0000,
                0b101 => cpu.registers.l = 0b0000_0000,
                0b111 => cpu.registers.a = 0b0000_0000,
                _ => panic!("never reach"),
            }
            cpu.registers.carry = false;
            cpu.registers.half_carry = false;
            cpu.registers.subtraction = false;
            cpu.registers.zero = false;
            cpu.execute();
            assert_eq!(cpu.registers.pc, pc + 2);
            assert_eq!(cpu.registers.carry, false);
            assert_eq!(cpu.registers.half_carry, false);
            assert_eq!(cpu.registers.subtraction, false);
            assert_eq!(cpu.registers.zero, false);
            match i {
                0b000 => assert_eq!(cpu.registers.b, 0b0000_0000),
                0b001 => assert_eq!(cpu.registers.c, 0b0000_0000),
                0b010 => assert_eq!(cpu.registers.d, 0b0000_0000),
                0b011 => assert_eq!(cpu.registers.e, 0b0000_0000),
                0b100 => assert_eq!(cpu.registers.h, 0b0000_0000),
                0b101 => assert_eq!(cpu.registers.l, 0b0000_0000),
                0b111 => assert_eq!(cpu.registers.a, 0b0000_0000),
                _ => panic!("never reach"),
            }

            // 0b1111_1111
            let pc = cpu.registers.pc;
            set_inst!(cpu, pc, 0xCB, opcode);
            match i {
                0b000 => cpu.registers.b = 0b1111_1111,
                0b001 => cpu.registers.c = 0b1111_1111,
                0b010 => cpu.registers.d = 0b1111_1111,
                0b011 => cpu.registers.e = 0b1111_1111,
                0b100 => cpu.registers.h = 0b1111_1111,
                0b101 => cpu.registers.l = 0b1111_1111,
                0b111 => cpu.registers.a = 0b1111_1111,
                _ => panic!("never reach"),
            }
            cpu.registers.carry = true;
            cpu.registers.half_carry = true;
            cpu.registers.subtraction = true;
            cpu.registers.zero = true;
            cpu.execute();
            assert_eq!(cpu.registers.pc, pc + 2);
            assert_eq!(cpu.registers.carry, true);
            assert_eq!(cpu.registers.half_carry, true);
            assert_eq!(cpu.registers.subtraction, true);
            assert_eq!(cpu.registers.zero, true);
            match i {
                0b000 => assert_eq!(cpu.registers.b, !(1 << b)),
                0b001 => assert_eq!(cpu.registers.c, !(1 << b)),
                0b010 => assert_eq!(cpu.registers.d, !(1 << b)),
                0b011 => assert_eq!(cpu.registers.e, !(1 << b)),
                0b100 => assert_eq!(cpu.registers.h, !(1 << b)),
                0b101 => assert_eq!(cpu.registers.l, !(1 << b)),
                0b111 => assert_eq!(cpu.registers.a, !(1 << b)),
                _ => panic!("never reach"),
            }
        }
    }
}

#[test]
fn op_res_rrn() {
    let mut cpu = CPU::new();

    let opcode_base = 0b10_000_110;
    for b in [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111].iter() {
        let opcode = opcode_base | (b << 3);

        // 0b0000_0000
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, 0xCB, opcode);
        cpu.registers.h = 3;
        cpu.registers.l = 2;
        cpu.mmu.byte_set(0x302, 0b0000_0000);
        cpu.registers.carry = false;
        cpu.registers.half_carry = false;
        cpu.registers.subtraction = false;
        cpu.registers.zero = false;
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 2);
        assert_eq!(cpu.registers.carry, false);
        assert_eq!(cpu.registers.half_carry, false);
        assert_eq!(cpu.registers.subtraction, false);
        assert_eq!(cpu.registers.zero, false);
        assert_eq!(cpu.mmu.byte_get(0x302), 0b0000_0000);

        // 0b1111_1111
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, 0xCB, opcode);
        cpu.registers.h = 4;
        cpu.registers.l = 3;
        cpu.mmu.byte_set(0x403, 0b1111_1111);
        cpu.registers.carry = true;
        cpu.registers.half_carry = true;
        cpu.registers.subtraction = true;
        cpu.registers.zero = true;
        cpu.execute();
        assert_eq!(cpu.registers.pc, pc + 2);
        assert_eq!(cpu.registers.carry, true);
        assert_eq!(cpu.registers.half_carry, true);
        assert_eq!(cpu.registers.subtraction, true);
        assert_eq!(cpu.registers.zero, true);
        match b {
            0b000 => assert_eq!(cpu.mmu.byte_get(0x403), 0b1111_1110),
            0b001 => assert_eq!(cpu.mmu.byte_get(0x403), 0b1111_1101),
            0b010 => assert_eq!(cpu.mmu.byte_get(0x403), 0b1111_1011),
            0b011 => assert_eq!(cpu.mmu.byte_get(0x403), 0b1111_0111),
            0b100 => assert_eq!(cpu.mmu.byte_get(0x403), 0b1110_1111),
            0b101 => assert_eq!(cpu.mmu.byte_get(0x403), 0b1101_1111),
            0b110 => assert_eq!(cpu.mmu.byte_get(0x403), 0b1011_1111),
            0b111 => assert_eq!(cpu.mmu.byte_get(0x403), 0b0111_1111),
            _ => panic!("never reach"),
        }
    }
}
