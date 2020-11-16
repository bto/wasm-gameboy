use super::*;

#[test]
fn test_new() {
    let cpu = CPU::new();
    assert_eq!(cpu.bus, MemoryBus::new());
    assert_eq!(cpu.registers, Registers::new());
}

#[test]
fn test_op_ld_a_nn() {
    let mut cpu = CPU::new();

    cpu.registers.pc = 0x100;
    cpu.bus.byte_set(0x101, 2);
    cpu.bus.byte_set(0x102, 3);
    cpu.bus.byte_set(0x302, 4);
    assert_eq!(cpu.execute(0b11111010), 0x103);
    assert_eq!(cpu.registers.a, 4);
}

#[test]
fn test_op_ld_a_rp() {
    let mut cpu = CPU::new();

    // LD A, (BC)
    cpu.registers.pc = 0x100;
    cpu.registers.b = 1;
    cpu.registers.c = 0;
    cpu.bus.byte_set(0x100, 2);
    assert_eq!(cpu.execute(0b00001010), 0x101);
    assert_eq!(cpu.registers.a, 2);

    // LD A, (DE)
    cpu.registers.pc = 0x101;
    cpu.registers.d = 2;
    cpu.registers.e = 1;
    cpu.bus.byte_set(0x201, 3);
    assert_eq!(cpu.execute(0b00011010), 0x102);
    assert_eq!(cpu.registers.a, 3);
}

#[test]
fn test_op_ld_nn_a() {
    let mut cpu = CPU::new();

    cpu.registers.pc = 0x100;
    cpu.bus.byte_set(0x101, 2);
    cpu.bus.byte_set(0x102, 3);
    cpu.registers.a = 4;
    assert_eq!(cpu.execute(0b11101010), 0x103);
    assert_eq!(cpu.bus.byte_get(0x302), 4);
}

#[test]
fn test_op_ld_r_n() {
    let mut cpu = CPU::new();

    // LD B, n
    cpu.registers.pc = 0x100;
    cpu.bus.byte_set(0x101, 2);
    assert_eq!(cpu.execute(0b00000110), 0x102);
    assert_eq!(cpu.registers.b, 2);

    // LD L, n
    cpu.registers.pc = 0x201;
    cpu.bus.byte_set(0x202, 3);
    assert_eq!(cpu.execute(0b00101110), 0x203);
    assert_eq!(cpu.registers.l, 3);

    // LD (HL), n
    cpu.registers.h = 1;
    cpu.registers.l = 0;
    cpu.registers.pc = 0x302;
    cpu.bus.byte_set(0x303, 4);
    assert_eq!(cpu.execute(0b00110110), 0x304);
    assert_eq!(cpu.bus.byte_get(0x100), 4);
}

#[test]
fn test_op_ld_r_r() {
    let mut cpu = CPU::new();

    // LD B, C
    cpu.registers.pc = 0x100;
    cpu.registers.b = 1;
    cpu.registers.c = 2;
    assert_eq!(cpu.execute(0b01000001), 0x101);
    assert_eq!(cpu.registers.b, 2);

    // LD E, D
    cpu.registers.pc = 0x201;
    cpu.registers.d = 3;
    cpu.registers.e = 4;
    assert_eq!(cpu.execute(0b01011010), 0x202);
    assert_eq!(cpu.registers.e, 3);

    // LD C, (HL)
    cpu.registers.pc = 0x302;
    cpu.registers.h = 1;
    cpu.registers.l = 0;
    cpu.bus.byte_set(0x100, 4);
    assert_eq!(cpu.execute(0b01001110), 0x303);
    assert_eq!(cpu.registers.c, 4);

    // LD (HL), D
    cpu.registers.pc = 0x403;
    cpu.registers.h = 2;
    cpu.registers.l = 1;
    cpu.registers.d = 5;
    assert_eq!(cpu.execute(0b01110010), 0x404);
    assert_eq!(cpu.bus.byte_get(0x201), 5);
}

#[test]
fn test_op_rp_a() {
    let mut cpu = CPU::new();

    // LD (BC), A
    cpu.registers.pc = 0x100;
    cpu.registers.b = 2;
    cpu.registers.c = 1;
    cpu.registers.a = 3;
    assert_eq!(cpu.execute(0b00000010), 0x101);
    assert_eq!(cpu.bus.byte_get(0x201), 3);

    // LD (DE), A
    cpu.registers.pc = 0x201;
    cpu.registers.d = 3;
    cpu.registers.e = 2;
    cpu.registers.a = 4;
    assert_eq!(cpu.execute(0b00010010), 0x202);
    assert_eq!(cpu.bus.byte_get(0x302), 4);
}

#[test]
fn test_register_16_get() {
    let mut cpu = CPU::new();

    // 00xxxx: BC
    cpu.registers.b = 1;
    cpu.registers.c = 0;
    assert_eq!(cpu.register_16_get(0b000000), 0x100);

    // 01xxxx: DE
    cpu.registers.d = 2;
    cpu.registers.e = 1;
    assert_eq!(cpu.register_16_get(0b010000), 0x201);

    // 10xxxx: HL
    cpu.registers.h = 3;
    cpu.registers.l = 2;
    assert_eq!(cpu.register_16_get(0b100000), 0x302);

    // 11xxxx: SP
    cpu.registers.sp = 0x403;
    assert_eq!(cpu.register_16_get(0b110000), 0x403);
}

#[test]
fn test_register_16_set() {
    let mut cpu = CPU::new();

    // 00xxxx: BC
    cpu.registers.b = 1;
    cpu.registers.c = 0;
    cpu.register_16_set(0b000000, 2);
    assert_eq!(cpu.bus.byte_get(0x100), 2);

    // 01xxxx: DE
    cpu.registers.d = 2;
    cpu.registers.e = 1;
    cpu.register_16_set(0b010000, 3);
    assert_eq!(cpu.bus.byte_get(0x201), 3);

    // 10xxxx: HL
    cpu.registers.h = 3;
    cpu.registers.l = 2;
    cpu.register_16_set(0b100000, 4);
    assert_eq!(cpu.bus.byte_get(0x302), 4);

    // 11xxxx: SP
    cpu.registers.sp = 0x403;
    cpu.register_16_set(0b110000, 5);
    assert_eq!(cpu.bus.byte_get(0x403), 5);
}

#[test]
fn test_register_8_get() {
    let mut cpu = CPU::new();

    // 000: B
    cpu.registers.b = 1;
    assert_eq!(cpu.register_8_get(0b000), 1);

    // 001: C
    cpu.registers.c = 2;
    assert_eq!(cpu.register_8_get(0b001), 2);

    // 010: D
    cpu.registers.d = 3;
    assert_eq!(cpu.register_8_get(0b010), 3);

    // 011: E
    cpu.registers.e = 4;
    assert_eq!(cpu.register_8_get(0b011), 4);

    // 100: H
    cpu.registers.h = 5;
    assert_eq!(cpu.register_8_get(0b100), 5);

    // 101: L
    cpu.registers.l = 6;
    assert_eq!(cpu.register_8_get(0b101), 6);

    // 110: (HL)
    cpu.registers.h = 1;
    cpu.registers.l = 0;
    cpu.bus.byte_set(0x100, 7);
    assert_eq!(cpu.register_8_get(0b110), 7);

    // 111: A
    cpu.registers.a = 8;
    assert_eq!(cpu.register_8_get(0b111), 8);
}

#[test]
fn test_register_8_set() {
    let mut cpu = CPU::new();

    // 000: B
    cpu.register_8_set(0b000000, 1);
    assert_eq!(cpu.registers.b, 1);

    // 001: C
    cpu.register_8_set(0b001000, 2);
    assert_eq!(cpu.registers.c, 2);

    // 010: D
    cpu.register_8_set(0b010000, 3);
    assert_eq!(cpu.registers.d, 3);

    // 011: E
    cpu.register_8_set(0b011000, 4);
    assert_eq!(cpu.registers.e, 4);

    // 100: H
    cpu.register_8_set(0b100000, 5);
    assert_eq!(cpu.registers.h, 5);

    // 101: L
    cpu.register_8_set(0b101000, 6);
    assert_eq!(cpu.registers.l, 6);

    // 110: (HL)
    cpu.registers.h = 1;
    cpu.registers.l = 0;
    cpu.register_8_set(0b110000, 7);
    assert_eq!(cpu.bus.byte_get(0x100), 7);

    // 111: A
    cpu.register_8_set(0b111000, 8);
    assert_eq!(cpu.registers.a, 8);
}
