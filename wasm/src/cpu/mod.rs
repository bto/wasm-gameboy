use super::mmu::MMU;

mod registers;
use registers::Registers;

#[macro_use]
mod opcode;

#[derive(Debug, PartialEq)]
pub struct CPU {
    halt: bool,
    int_disable: bool,
    int_enable: bool,
    mmu: MMU,
    registers: Registers,
    stop: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            halt: false,
            int_disable: false,
            int_enable: false,
            mmu: MMU::new(),
            registers: Registers::new(),
            stop: false,
        }
    }

    pub fn execute(&mut self) {
        #[allow(clippy::inconsistent_digit_grouping)]
        match self.fetch_byte() {
            0b00_000_110 => op_ld_r_n!(self, b),
            0b00_001_110 => op_ld_r_n!(self, c),
            0b00_010_110 => op_ld_r_n!(self, d),
            0b00_011_110 => op_ld_r_n!(self, e),
            0b00_100_110 => op_ld_r_n!(self, h),
            0b00_101_110 => op_ld_r_n!(self, l),
            0b00_110_110 => op_ld_rrn_n!(self, hl),
            0b00_111_110 => op_ld_r_n!(self, a),

            0b01_000_000 => {}
            0b01_000_001 => op_ld_r_r!(self, b, c),
            0b01_000_010 => op_ld_r_r!(self, b, d),
            0b01_000_011 => op_ld_r_r!(self, b, e),
            0b01_000_100 => op_ld_r_r!(self, b, h),
            0b01_000_101 => op_ld_r_r!(self, b, l),
            0b01_000_110 => op_ld_r_rrn!(self, b, hl),
            0b01_000_111 => op_ld_r_r!(self, b, a),

            0b01_001_000 => op_ld_r_r!(self, c, b),
            0b01_001_001 => {}
            0b01_001_010 => op_ld_r_r!(self, c, d),
            0b01_001_011 => op_ld_r_r!(self, c, e),
            0b01_001_100 => op_ld_r_r!(self, c, h),
            0b01_001_101 => op_ld_r_r!(self, c, l),
            0b01_001_110 => op_ld_r_rrn!(self, c, hl),
            0b01_001_111 => op_ld_r_r!(self, c, a),

            0b01_010_000 => op_ld_r_r!(self, d, b),
            0b01_010_001 => op_ld_r_r!(self, d, c),
            0b01_010_010 => {}
            0b01_010_011 => op_ld_r_r!(self, d, e),
            0b01_010_100 => op_ld_r_r!(self, d, h),
            0b01_010_101 => op_ld_r_r!(self, d, l),
            0b01_010_110 => op_ld_r_rrn!(self, d, hl),
            0b01_010_111 => op_ld_r_r!(self, d, a),

            0b01_011_000 => op_ld_r_r!(self, e, b),
            0b01_011_001 => op_ld_r_r!(self, e, c),
            0b01_011_010 => op_ld_r_r!(self, e, d),
            0b01_011_011 => {}
            0b01_011_100 => op_ld_r_r!(self, e, h),
            0b01_011_101 => op_ld_r_r!(self, e, l),
            0b01_011_110 => op_ld_r_rrn!(self, e, hl),
            0b01_011_111 => op_ld_r_r!(self, e, a),

            0b01_100_000 => op_ld_r_r!(self, h, b),
            0b01_100_001 => op_ld_r_r!(self, h, c),
            0b01_100_010 => op_ld_r_r!(self, h, d),
            0b01_100_011 => op_ld_r_r!(self, h, e),
            0b01_100_100 => {}
            0b01_100_101 => op_ld_r_r!(self, h, l),
            0b01_100_110 => op_ld_r_rrn!(self, h, hl),
            0b01_100_111 => op_ld_r_r!(self, h, a),

            0b01_101_000 => op_ld_r_r!(self, l, b),
            0b01_101_001 => op_ld_r_r!(self, l, c),
            0b01_101_010 => op_ld_r_r!(self, l, d),
            0b01_101_011 => op_ld_r_r!(self, l, e),
            0b01_101_100 => op_ld_r_r!(self, l, h),
            0b01_101_101 => {}
            0b01_101_110 => op_ld_r_rrn!(self, l, hl),
            0b01_101_111 => op_ld_r_r!(self, l, a),

            0b01_110_000 => op_ld_rrn_r!(self, hl, b),
            0b01_110_001 => op_ld_rrn_r!(self, hl, c),
            0b01_110_010 => op_ld_rrn_r!(self, hl, d),
            0b01_110_011 => op_ld_rrn_r!(self, hl, e),
            0b01_110_100 => op_ld_rrn_r!(self, hl, h),
            0b01_110_101 => op_ld_rrn_r!(self, hl, l),
            0b01_110_111 => op_ld_rrn_r!(self, hl, a),

            0b01_111_000 => op_ld_r_r!(self, a, b),
            0b01_111_001 => op_ld_r_r!(self, a, c),
            0b01_111_010 => op_ld_r_r!(self, a, d),
            0b01_111_011 => op_ld_r_r!(self, a, e),
            0b01_111_100 => op_ld_r_r!(self, a, h),
            0b01_111_101 => op_ld_r_r!(self, a, l),
            0b01_111_110 => op_ld_r_rrn!(self, a, hl),
            0b01_111_111 => {}

            0b00_00_1010 => op_ld_r_rrn!(self, a, bc),
            0b00_01_1010 => op_ld_r_rrn!(self, a, de),
            0b00_10_1010 => op_ldi_r_rr!(self, a, hl),
            0b00_11_1010 => op_ldd_r_rr!(self, a, hl),

            0b00_00_0010 => op_ld_rrn_r!(self, bc, a),
            0b00_01_0010 => op_ld_rrn_r!(self, de, a),
            0b00_10_0010 => op_ldi_rr_r!(self, hl, a),
            0b00_11_0010 => op_ldd_rr_r!(self, hl, a),

            0b111_0_1010 => op_ld_nn_r!(self, a),
            0b111_1_1010 => op_ld_r_nn!(self, a),

            0b111_0_0010 => op_ldh_rh_r!(self, c, a),
            0b111_1_0010 => op_ldh_r_rh!(self, a, c),

            0b111_0_0000 => op_ldh_nh_r!(self, a),
            0b111_1_0000 => op_ldh_r_nh!(self, a),

            0b00_00_0001 => op_ld_rr_nn!(self, bc),
            0b00_01_0001 => op_ld_rr_nn!(self, de),
            0b00_10_0001 => op_ld_rr_nn!(self, hl),
            0b00_11_0001 => op_ld_rr_nn!(self, sp),

            0b0000_1000 => op_ld_nn_rr!(self, sp),
            0b1111_1001 => op_ld_rr_rr!(self, sp, hl),

            0b11_00_0101 => op_push_rr!(self, bc),
            0b11_01_0101 => op_push_rr!(self, de),
            0b11_10_0101 => op_push_rr!(self, hl),
            0b11_11_0101 => op_push_rr!(self, af),

            0b11_00_0001 => op_pop_rr!(self, bc),
            0b11_01_0001 => op_pop_rr!(self, de),
            0b11_10_0001 => op_pop_rr!(self, hl),
            0b11_11_0001 => op_pop_rr!(self, af),

            0b10000_000 => op_add_r_r!(self, a, b),
            0b10000_001 => op_add_r_r!(self, a, c),
            0b10000_010 => op_add_r_r!(self, a, d),
            0b10000_011 => op_add_r_r!(self, a, e),
            0b10000_100 => op_add_r_r!(self, a, h),
            0b10000_101 => op_add_r_r!(self, a, l),
            0b10000_110 => op_add_r_rrn!(self, a, hl),
            0b10000_111 => op_add_r_r!(self, a, a),

            0b10001_000 => op_adc_r_r!(self, a, b),
            0b10001_001 => op_adc_r_r!(self, a, c),
            0b10001_010 => op_adc_r_r!(self, a, d),
            0b10001_011 => op_adc_r_r!(self, a, e),
            0b10001_100 => op_adc_r_r!(self, a, h),
            0b10001_101 => op_adc_r_r!(self, a, l),
            0b10001_110 => op_adc_r_rrn!(self, a, hl),
            0b10001_111 => op_adc_r_r!(self, a, a),

            0b10010_000 => op_sub_r_r!(self, a, b),
            0b10010_001 => op_sub_r_r!(self, a, c),
            0b10010_010 => op_sub_r_r!(self, a, d),
            0b10010_011 => op_sub_r_r!(self, a, e),
            0b10010_100 => op_sub_r_r!(self, a, h),
            0b10010_101 => op_sub_r_r!(self, a, l),
            0b10010_110 => op_sub_r_rrn!(self, a, hl),
            0b10010_111 => op_sub_r_r!(self, a, a),

            0b10011_000 => op_sbc_r_r!(self, a, b),
            0b10011_001 => op_sbc_r_r!(self, a, c),
            0b10011_010 => op_sbc_r_r!(self, a, d),
            0b10011_011 => op_sbc_r_r!(self, a, e),
            0b10011_100 => op_sbc_r_r!(self, a, h),
            0b10011_101 => op_sbc_r_r!(self, a, l),
            0b10011_110 => op_sbc_r_rrn!(self, a, hl),
            0b10011_111 => op_sbc_r_r!(self, a, a),

            0b10100_000 => op_and_r_r!(self, a, b),
            0b10100_001 => op_and_r_r!(self, a, c),
            0b10100_010 => op_and_r_r!(self, a, d),
            0b10100_011 => op_and_r_r!(self, a, e),
            0b10100_100 => op_and_r_r!(self, a, h),
            0b10100_101 => op_and_r_r!(self, a, l),
            0b10100_110 => op_and_r_rrn!(self, a, hl),
            0b10100_111 => op_and_r_r!(self, a, a),

            0b10101_000 => op_xor_r_r!(self, a, b),
            0b10101_001 => op_xor_r_r!(self, a, c),
            0b10101_010 => op_xor_r_r!(self, a, d),
            0b10101_011 => op_xor_r_r!(self, a, e),
            0b10101_100 => op_xor_r_r!(self, a, h),
            0b10101_101 => op_xor_r_r!(self, a, l),
            0b10101_110 => op_xor_r_rrn!(self, a, hl),
            0b10101_111 => op_xor_r_r!(self, a, a),

            0b10110_000 => op_or_r_r!(self, a, b),
            0b10110_001 => op_or_r_r!(self, a, c),
            0b10110_010 => op_or_r_r!(self, a, d),
            0b10110_011 => op_or_r_r!(self, a, e),
            0b10110_100 => op_or_r_r!(self, a, h),
            0b10110_101 => op_or_r_r!(self, a, l),
            0b10110_110 => op_or_r_rrn!(self, a, hl),
            0b10110_111 => op_or_r_r!(self, a, a),

            0b10111_000 => op_cp_r_r!(self, a, b),
            0b10111_001 => op_cp_r_r!(self, a, c),
            0b10111_010 => op_cp_r_r!(self, a, d),
            0b10111_011 => op_cp_r_r!(self, a, e),
            0b10111_100 => op_cp_r_r!(self, a, h),
            0b10111_101 => op_cp_r_r!(self, a, l),
            0b10111_110 => op_cp_r_rrn!(self, a, hl),
            0b10111_111 => op_cp_r_r!(self, a, a),

            0b00_000_100 => op_inc_r!(self, b),
            0b00_001_100 => op_inc_r!(self, c),
            0b00_010_100 => op_inc_r!(self, d),
            0b00_011_100 => op_inc_r!(self, e),
            0b00_100_100 => op_inc_r!(self, h),
            0b00_101_100 => op_inc_r!(self, l),
            0b00_110_100 => op_inc_rrn!(self, hl),
            0b00_111_100 => op_inc_r!(self, a),

            0b00_000_101 => op_dec_r!(self, b),
            0b00_001_101 => op_dec_r!(self, c),
            0b00_010_101 => op_dec_r!(self, d),
            0b00_011_101 => op_dec_r!(self, e),
            0b00_100_101 => op_dec_r!(self, h),
            0b00_101_101 => op_dec_r!(self, l),
            0b00_110_101 => op_dec_rrn!(self, hl),
            0b00_111_101 => op_dec_r!(self, a),

            0b11_000_110 => op_add_r_n!(self, a),
            0b11_001_110 => op_adc_r_n!(self, a),
            0b11_010_110 => op_sub_r_n!(self, a),
            0b11_011_110 => op_sbc_r_n!(self, a),
            0b11_100_110 => op_and_r_n!(self, a),
            0b11_101_110 => op_xor_r_n!(self, a),
            0b11_110_110 => op_or_r_n!(self, a),
            0b11_111_110 => op_cp_r_n!(self, a),

            0b00_00_1001 => op_add_rr_rr!(self, hl, bc),
            0b00_01_1001 => op_add_rr_rr!(self, hl, de),
            0b00_10_1001 => op_add_rr_rr!(self, hl, hl),
            0b00_11_1001 => op_add_rr_rr!(self, hl, sp),

            0b1110_1000 => op_add_rr_rr_n!(self, sp, sp),
            0b1111_1000 => op_add_rr_rr_n!(self, hl, sp),

            0b00_00_0011 => op_inc_rr!(self, bc),
            0b00_01_0011 => op_inc_rr!(self, de),
            0b00_10_0011 => op_inc_rr!(self, hl),
            0b00_11_0011 => op_inc_rr!(self, sp),

            0b00_00_1011 => op_dec_rr!(self, bc),
            0b00_01_1011 => op_dec_rr!(self, de),
            0b00_10_1011 => op_dec_rr!(self, hl),
            0b00_11_1011 => op_dec_rr!(self, sp),

            0b0010_0111 => op_daa_r!(self, a),
            0b0010_1111 => op_cpl_r!(self, a),
            0b0011_1111 => op_ccf!(self),
            0b0011_0111 => op_scf!(self),
            0b0000_0000 => {} // NOP
            0b0111_0110 => op_halt!(self),
            0b0001_0000 => op_stop!(self),
            0b1111_0011 => op_int_disable!(self),
            0b1111_1011 => op_int_enable!(self),

            0b0000_0111 => op_rlc_r!(self, a),
            0b0001_0111 => op_rl_r!(self, a),
            0b0000_1111 => op_rrc_r!(self, a),
            0b0001_1111 => op_rr_r!(self, a),

            0b11000011 => op_jp_nn!(self),
            0b11101001 => op_jp_rr!(self, hl),

            0b110_00_010 => op_jp_nn_nz!(self),
            0b110_01_010 => op_jp_nn_z!(self),
            0b110_10_010 => op_jp_nn_nc!(self),
            0b110_11_010 => op_jp_nn_c!(self),

            0b00011000 => op_jr_n!(self),

            0b001_00_000 => op_jr_n_nz!(self),
            0b001_01_000 => op_jr_n_z!(self),
            0b001_10_000 => op_jr_n_nc!(self),
            0b001_11_000 => op_jr_n_c!(self),

            0b11001101 => op_call_nn!(self),

            0b110_00_100 => op_call_nn_nz!(self),
            0b110_01_100 => op_call_nn_z!(self),
            0b110_10_100 => op_call_nn_nc!(self),
            0b110_11_100 => op_call_nn_c!(self),

            0b11_000_111 => op_rst!(self, 0x00),
            0b11_001_111 => op_rst!(self, 0x08),
            0b11_010_111 => op_rst!(self, 0x10),
            0b11_011_111 => op_rst!(self, 0x18),
            0b11_100_111 => op_rst!(self, 0x20),
            0b11_101_111 => op_rst!(self, 0x28),
            0b11_110_111 => op_rst!(self, 0x30),
            0b11_111_111 => op_rst!(self, 0x38),

            0b11001001 => op_ret!(self),
            0b11011001 => op_reti!(self),

            0b110_00_000 => op_ret_nz!(self),
            0b110_01_000 => op_ret_z!(self),
            0b110_10_000 => op_ret_nc!(self),
            0b110_11_000 => op_ret_c!(self),

            0xCB => self.execute_cb(),

            inst => panic!("Instruction {:2X} is not a valid instruction", inst),
        }
    }

    fn execute_cb(&mut self) {
        #[allow(clippy::inconsistent_digit_grouping)]
        match self.fetch_byte() {
            0b00000_000 => op_rlc_r!(self, b),
            0b00000_001 => op_rlc_r!(self, c),
            0b00000_010 => op_rlc_r!(self, d),
            0b00000_011 => op_rlc_r!(self, e),
            0b00000_100 => op_rlc_r!(self, h),
            0b00000_101 => op_rlc_r!(self, l),
            0b00000_110 => op_rlc_rrn!(self, hl),
            0b00000_111 => op_rlc_r!(self, a),

            0b00001_000 => op_rrc_r!(self, b),
            0b00001_001 => op_rrc_r!(self, c),
            0b00001_010 => op_rrc_r!(self, d),
            0b00001_011 => op_rrc_r!(self, e),
            0b00001_100 => op_rrc_r!(self, h),
            0b00001_101 => op_rrc_r!(self, l),
            0b00001_110 => op_rrc_rrn!(self, hl),
            0b00001_111 => op_rrc_r!(self, a),

            0b00010_000 => op_rl_r!(self, b),
            0b00010_001 => op_rl_r!(self, c),
            0b00010_010 => op_rl_r!(self, d),
            0b00010_011 => op_rl_r!(self, e),
            0b00010_100 => op_rl_r!(self, h),
            0b00010_101 => op_rl_r!(self, l),
            0b00010_110 => op_rl_rrn!(self, hl),
            0b00010_111 => op_rl_r!(self, a),

            0b00011_000 => op_rr_r!(self, b),
            0b00011_001 => op_rr_r!(self, c),
            0b00011_010 => op_rr_r!(self, d),
            0b00011_011 => op_rr_r!(self, e),
            0b00011_100 => op_rr_r!(self, h),
            0b00011_101 => op_rr_r!(self, l),
            0b00011_110 => op_rr_rrn!(self, hl),
            0b00011_111 => op_rr_r!(self, a),

            0b00100_000 => op_sla_r!(self, b),
            0b00100_001 => op_sla_r!(self, c),
            0b00100_010 => op_sla_r!(self, d),
            0b00100_011 => op_sla_r!(self, e),
            0b00100_100 => op_sla_r!(self, h),
            0b00100_101 => op_sla_r!(self, l),
            0b00100_110 => op_sla_rrn!(self, hl),
            0b00100_111 => op_sla_r!(self, a),

            0b00101_000 => op_sra_r!(self, b),
            0b00101_001 => op_sra_r!(self, c),
            0b00101_010 => op_sra_r!(self, d),
            0b00101_011 => op_sra_r!(self, e),
            0b00101_100 => op_sra_r!(self, h),
            0b00101_101 => op_sra_r!(self, l),
            0b00101_110 => op_sra_rrn!(self, hl),
            0b00101_111 => op_sra_r!(self, a),

            0b00110_000 => op_swap_r!(self, b),
            0b00110_001 => op_swap_r!(self, c),
            0b00110_010 => op_swap_r!(self, d),
            0b00110_011 => op_swap_r!(self, e),
            0b00110_100 => op_swap_r!(self, h),
            0b00110_101 => op_swap_r!(self, l),
            0b00110_110 => op_swap_rrn!(self, hl),
            0b00110_111 => op_swap_r!(self, a),

            0b00111_000 => op_srl_r!(self, b),
            0b00111_001 => op_srl_r!(self, c),
            0b00111_010 => op_srl_r!(self, d),
            0b00111_011 => op_srl_r!(self, e),
            0b00111_100 => op_srl_r!(self, h),
            0b00111_101 => op_srl_r!(self, l),
            0b00111_110 => op_srl_rrn!(self, hl),
            0b00111_111 => op_srl_r!(self, a),

            0b01_000_000 => op_bit_r!(self, b, 0),
            0b01_000_001 => op_bit_r!(self, c, 0),
            0b01_000_010 => op_bit_r!(self, d, 0),
            0b01_000_011 => op_bit_r!(self, e, 0),
            0b01_000_100 => op_bit_r!(self, h, 0),
            0b01_000_101 => op_bit_r!(self, l, 0),
            0b01_000_110 => op_bit_rrn!(self, hl, 0),
            0b01_000_111 => op_bit_r!(self, a, 0),

            0b01_001_000 => op_bit_r!(self, b, 1),
            0b01_001_001 => op_bit_r!(self, c, 1),
            0b01_001_010 => op_bit_r!(self, d, 1),
            0b01_001_011 => op_bit_r!(self, e, 1),
            0b01_001_100 => op_bit_r!(self, h, 1),
            0b01_001_101 => op_bit_r!(self, l, 1),
            0b01_001_110 => op_bit_rrn!(self, hl, 1),
            0b01_001_111 => op_bit_r!(self, a, 1),

            0b01_010_000 => op_bit_r!(self, b, 2),
            0b01_010_001 => op_bit_r!(self, c, 2),
            0b01_010_010 => op_bit_r!(self, d, 2),
            0b01_010_011 => op_bit_r!(self, e, 2),
            0b01_010_100 => op_bit_r!(self, h, 2),
            0b01_010_101 => op_bit_r!(self, l, 2),
            0b01_010_110 => op_bit_rrn!(self, hl, 2),
            0b01_010_111 => op_bit_r!(self, a, 2),

            0b01_011_000 => op_bit_r!(self, b, 3),
            0b01_011_001 => op_bit_r!(self, c, 3),
            0b01_011_010 => op_bit_r!(self, d, 3),
            0b01_011_011 => op_bit_r!(self, e, 3),
            0b01_011_100 => op_bit_r!(self, h, 3),
            0b01_011_101 => op_bit_r!(self, l, 3),
            0b01_011_110 => op_bit_rrn!(self, hl, 3),
            0b01_011_111 => op_bit_r!(self, a, 3),

            0b01_100_000 => op_bit_r!(self, b, 4),
            0b01_100_001 => op_bit_r!(self, c, 4),
            0b01_100_010 => op_bit_r!(self, d, 4),
            0b01_100_011 => op_bit_r!(self, e, 4),
            0b01_100_100 => op_bit_r!(self, h, 4),
            0b01_100_101 => op_bit_r!(self, l, 4),
            0b01_100_110 => op_bit_rrn!(self, hl, 4),
            0b01_100_111 => op_bit_r!(self, a, 4),

            0b01_101_000 => op_bit_r!(self, b, 5),
            0b01_101_001 => op_bit_r!(self, c, 5),
            0b01_101_010 => op_bit_r!(self, d, 5),
            0b01_101_011 => op_bit_r!(self, e, 5),
            0b01_101_100 => op_bit_r!(self, h, 5),
            0b01_101_101 => op_bit_r!(self, l, 5),
            0b01_101_110 => op_bit_rrn!(self, hl, 5),
            0b01_101_111 => op_bit_r!(self, a, 5),

            0b01_110_000 => op_bit_r!(self, b, 6),
            0b01_110_001 => op_bit_r!(self, c, 6),
            0b01_110_010 => op_bit_r!(self, d, 6),
            0b01_110_011 => op_bit_r!(self, e, 6),
            0b01_110_100 => op_bit_r!(self, h, 6),
            0b01_110_101 => op_bit_r!(self, l, 6),
            0b01_110_110 => op_bit_rrn!(self, hl, 6),
            0b01_110_111 => op_bit_r!(self, a, 6),

            0b01_111_000 => op_bit_r!(self, b, 7),
            0b01_111_001 => op_bit_r!(self, c, 7),
            0b01_111_010 => op_bit_r!(self, d, 7),
            0b01_111_011 => op_bit_r!(self, e, 7),
            0b01_111_100 => op_bit_r!(self, h, 7),
            0b01_111_101 => op_bit_r!(self, l, 7),
            0b01_111_110 => op_bit_rrn!(self, hl, 7),
            0b01_111_111 => op_bit_r!(self, a, 7),

            0b10_000_000 => op_res_r!(self, b, 0),
            0b10_000_001 => op_res_r!(self, c, 0),
            0b10_000_010 => op_res_r!(self, d, 0),
            0b10_000_011 => op_res_r!(self, e, 0),
            0b10_000_100 => op_res_r!(self, h, 0),
            0b10_000_101 => op_res_r!(self, l, 0),
            0b10_000_110 => op_res_rrn!(self, hl, 0),
            0b10_000_111 => op_res_r!(self, a, 0),

            0b10_001_000 => op_res_r!(self, b, 1),
            0b10_001_001 => op_res_r!(self, c, 1),
            0b10_001_010 => op_res_r!(self, d, 1),
            0b10_001_011 => op_res_r!(self, e, 1),
            0b10_001_100 => op_res_r!(self, h, 1),
            0b10_001_101 => op_res_r!(self, l, 1),
            0b10_001_110 => op_res_rrn!(self, hl, 1),
            0b10_001_111 => op_res_r!(self, a, 1),

            0b10_010_000 => op_res_r!(self, b, 2),
            0b10_010_001 => op_res_r!(self, c, 2),
            0b10_010_010 => op_res_r!(self, d, 2),
            0b10_010_011 => op_res_r!(self, e, 2),
            0b10_010_100 => op_res_r!(self, h, 2),
            0b10_010_101 => op_res_r!(self, l, 2),
            0b10_010_110 => op_res_rrn!(self, hl, 2),
            0b10_010_111 => op_res_r!(self, a, 2),

            0b10_011_000 => op_res_r!(self, b, 3),
            0b10_011_001 => op_res_r!(self, c, 3),
            0b10_011_010 => op_res_r!(self, d, 3),
            0b10_011_011 => op_res_r!(self, e, 3),
            0b10_011_100 => op_res_r!(self, h, 3),
            0b10_011_101 => op_res_r!(self, l, 3),
            0b10_011_110 => op_res_rrn!(self, hl, 3),
            0b10_011_111 => op_res_r!(self, a, 3),

            0b10_100_000 => op_res_r!(self, b, 4),
            0b10_100_001 => op_res_r!(self, c, 4),
            0b10_100_010 => op_res_r!(self, d, 4),
            0b10_100_011 => op_res_r!(self, e, 4),
            0b10_100_100 => op_res_r!(self, h, 4),
            0b10_100_101 => op_res_r!(self, l, 4),
            0b10_100_110 => op_res_rrn!(self, hl, 4),
            0b10_100_111 => op_res_r!(self, a, 4),

            0b10_101_000 => op_res_r!(self, b, 5),
            0b10_101_001 => op_res_r!(self, c, 5),
            0b10_101_010 => op_res_r!(self, d, 5),
            0b10_101_011 => op_res_r!(self, e, 5),
            0b10_101_100 => op_res_r!(self, h, 5),
            0b10_101_101 => op_res_r!(self, l, 5),
            0b10_101_110 => op_res_rrn!(self, hl, 5),
            0b10_101_111 => op_res_r!(self, a, 5),

            0b10_110_000 => op_res_r!(self, b, 6),
            0b10_110_001 => op_res_r!(self, c, 6),
            0b10_110_010 => op_res_r!(self, d, 6),
            0b10_110_011 => op_res_r!(self, e, 6),
            0b10_110_100 => op_res_r!(self, h, 6),
            0b10_110_101 => op_res_r!(self, l, 6),
            0b10_110_110 => op_res_rrn!(self, hl, 6),
            0b10_110_111 => op_res_r!(self, a, 6),

            0b10_111_000 => op_res_r!(self, b, 7),
            0b10_111_001 => op_res_r!(self, c, 7),
            0b10_111_010 => op_res_r!(self, d, 7),
            0b10_111_011 => op_res_r!(self, e, 7),
            0b10_111_100 => op_res_r!(self, h, 7),
            0b10_111_101 => op_res_r!(self, l, 7),
            0b10_111_110 => op_res_rrn!(self, hl, 7),
            0b10_111_111 => op_res_r!(self, a, 7),

            0b11_000_000 => op_set_r!(self, b, 0),
            0b11_000_001 => op_set_r!(self, c, 0),
            0b11_000_010 => op_set_r!(self, d, 0),
            0b11_000_011 => op_set_r!(self, e, 0),
            0b11_000_100 => op_set_r!(self, h, 0),
            0b11_000_101 => op_set_r!(self, l, 0),
            0b11_000_110 => op_set_rrn!(self, hl, 0),
            0b11_000_111 => op_set_r!(self, a, 0),

            0b11_001_000 => op_set_r!(self, b, 1),
            0b11_001_001 => op_set_r!(self, c, 1),
            0b11_001_010 => op_set_r!(self, d, 1),
            0b11_001_011 => op_set_r!(self, e, 1),
            0b11_001_100 => op_set_r!(self, h, 1),
            0b11_001_101 => op_set_r!(self, l, 1),
            0b11_001_110 => op_set_rrn!(self, hl, 1),
            0b11_001_111 => op_set_r!(self, a, 1),

            0b11_010_000 => op_set_r!(self, b, 2),
            0b11_010_001 => op_set_r!(self, c, 2),
            0b11_010_010 => op_set_r!(self, d, 2),
            0b11_010_011 => op_set_r!(self, e, 2),
            0b11_010_100 => op_set_r!(self, h, 2),
            0b11_010_101 => op_set_r!(self, l, 2),
            0b11_010_110 => op_set_rrn!(self, hl, 2),
            0b11_010_111 => op_set_r!(self, a, 2),

            0b11_011_000 => op_set_r!(self, b, 3),
            0b11_011_001 => op_set_r!(self, c, 3),
            0b11_011_010 => op_set_r!(self, d, 3),
            0b11_011_011 => op_set_r!(self, e, 3),
            0b11_011_100 => op_set_r!(self, h, 3),
            0b11_011_101 => op_set_r!(self, l, 3),
            0b11_011_110 => op_set_rrn!(self, hl, 3),
            0b11_011_111 => op_set_r!(self, a, 3),

            0b11_100_000 => op_set_r!(self, b, 4),
            0b11_100_001 => op_set_r!(self, c, 4),
            0b11_100_010 => op_set_r!(self, d, 4),
            0b11_100_011 => op_set_r!(self, e, 4),
            0b11_100_100 => op_set_r!(self, h, 4),
            0b11_100_101 => op_set_r!(self, l, 4),
            0b11_100_110 => op_set_rrn!(self, hl, 4),
            0b11_100_111 => op_set_r!(self, a, 4),

            0b11_101_000 => op_set_r!(self, b, 5),
            0b11_101_001 => op_set_r!(self, c, 5),
            0b11_101_010 => op_set_r!(self, d, 5),
            0b11_101_011 => op_set_r!(self, e, 5),
            0b11_101_100 => op_set_r!(self, h, 5),
            0b11_101_101 => op_set_r!(self, l, 5),
            0b11_101_110 => op_set_rrn!(self, hl, 5),
            0b11_101_111 => op_set_r!(self, a, 5),

            0b11_110_000 => op_set_r!(self, b, 6),
            0b11_110_001 => op_set_r!(self, c, 6),
            0b11_110_010 => op_set_r!(self, d, 6),
            0b11_110_011 => op_set_r!(self, e, 6),
            0b11_110_100 => op_set_r!(self, h, 6),
            0b11_110_101 => op_set_r!(self, l, 6),
            0b11_110_110 => op_set_rrn!(self, hl, 6),
            0b11_110_111 => op_set_r!(self, a, 6),

            0b11_111_000 => op_set_r!(self, b, 7),
            0b11_111_001 => op_set_r!(self, c, 7),
            0b11_111_010 => op_set_r!(self, d, 7),
            0b11_111_011 => op_set_r!(self, e, 7),
            0b11_111_100 => op_set_r!(self, h, 7),
            0b11_111_101 => op_set_r!(self, l, 7),
            0b11_111_110 => op_set_rrn!(self, hl, 7),
            0b11_111_111 => op_set_r!(self, a, 7),
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.mmu.byte_get(self.registers.pc);
        self.registers.pc += 1;
        byte
    }

    fn fetch_load(&mut self) -> u8 {
        let value = self.fetch_word();
        self.mmu.byte_get(value)
    }

    fn fetch_store(&mut self, value: u8) {
        let addr = self.fetch_word();
        self.mmu.byte_set(addr, value)
    }

    fn fetch_word(&mut self) -> u16 {
        let lsb = self.mmu.byte_get(self.registers.pc);
        self.registers.pc += 1;
        let msb = self.mmu.byte_get(self.registers.pc);
        self.registers.pc += 1;
        (msb as u16) << 8 | lsb as u16
    }
}

#[path = "./mod_tests.rs"]
#[cfg(test)]
mod tests;

#[path = "./opcode/tests.rs"]
#[cfg(test)]
mod opcode_tests;
