use super::*;

#[macro_use]
mod tests_macro;

#[test]
fn op_rst() {
    let mut cpu = CPU::new();

    let opcode_base = 0b11_000_111;
    for i in [0b000, 0b001, 0b010, 0b011, 0b100, 0b101, 0b110, 0b111].iter() {
        let opcode = opcode_base | i << 3;

        cpu.registers.pc = 0x123;
        let pc = cpu.registers.pc;
        set_inst!(cpu, pc, opcode);
        cpu.registers.sp = 0x234;
        cpu.execute();
        assert_eq!(cpu.registers.pc, (i * 8) as u16);
        assert_eq!(cpu.registers.sp, 0x232);
        assert_eq!(cpu.mmu.byte_get(0x233), 0x01);
        assert_eq!(cpu.mmu.byte_get(0x232), 0x24);
    }
}
