use super::*;

#[macro_use]
mod tests_macro;

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
