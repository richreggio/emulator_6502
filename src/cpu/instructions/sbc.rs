use super::*;

// SBC - Subtract Memory from Accumulator with Borrow
// Operation: A - M - ~C → A
// This instruction subtracts the value of memory and borrow from the value of the accumulator, using two's complement arithmetic, and stores the result in the accumulator. Borrow is defined as the carry flag complemented; therefore, a resultant carry flag indicates that a borrow has not occurred.
// This instruction affects the accumulator. The carry flag is set if the result is greater than or equal to 0. The carry flag is reset when the result is less than 0, indicating a borrow. The over­flow flag is set when the result exceeds +127 or -127, otherwise it is reset. The negative flag is set if the result in the accumulator has bit 7 on, otherwise it is reset. The Z flag is set if the result in the accumulator is 0, otherwise it is reset.
// In decimal mode, the N, V and Z flags are not consistent with the decimal result.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Immediate                      | SBC #$nn               | $E9    | 2 	      | 2          |
// | Immediate                      | SBC #$nn               | $EB*   | 2 	      | 2          |
// | Absolute                       | SBC $nnnn              | $ED    |	3         | 4          |
// | Absolute X-Indexed             | SBC $nnnn,X            | $FD    | 3         | 4+p        |
// | Absolute Y-Indexed	            | SBC $nnnn,Y            | $F9    | 3         | 4+p        |
// | Zero Page                      | SBC $nn                | $E5    | 2         | 3          |
// | Zero Page X-Indexed            | SBC $nn,X              | $F5    | 2         | 4          |
// | Zero Page X-Indexed Indirect   | SBC ($nn,X)            | $E1    | 2         | 6          |
// | Zero Page Y-Indexed Indirect   | SBC ($nn),Y            | $F1    | 2         | 5+p        |
// | p: =1 if page is crossed       |                        |        |           |            |
// |--------------------------------------------------------------------------------------------

pub fn sbc(memory: &mut Memory, registers: &mut Registers, operation: Operation) {
    let tmp_value = match operation.addressing_mode {
        AdMode::Immediate(address) => memory.read_byte(address),
        AdMode::Absolute(address) => memory.read_byte(address),
        AdMode::AbsoluteXIndex(address) => memory.read_byte(address),
        AdMode::AbsoluteYIndex(address) => memory.read_byte(address),
        AdMode::ZeroPage(address) => memory.read_byte(address),
        AdMode::ZeroPageXIndex(address) => memory.read_byte(address),
        AdMode::ZeroPageXIndexIndirect(address) => memory.read_byte(address),
        AdMode::ZeroPageYIndexIndirect(address) => memory.read_byte(address),
        _ => 0,
    };

    let (mut total, mut carry) = registers.accumulator.overflowing_sub(tmp_value);

    if !registers.carry_flag_is_set() && total != 0 {
        total -= 1;
    } else if !registers.carry_flag_is_set() {
        total = 0xFF;
        carry = true;
    }

    if carry {
        registers.set_carry_flag(true);
    } else {
        registers.set_carry_flag(false);
    }

    if ((registers.accumulator & 0b1000_0000) ^ (total & 0b1000_0000))
        & !((registers.accumulator & 0b1000_0000) ^ (tmp_value & 0b1000_0000))
        == 0b1000_0000
    {
        registers.set_overflow_flag(true);
    } else {
        registers.set_overflow_flag(false);
    }

    if total == 0 {
        registers.set_zero_flag(true);
    } else {
        registers.set_zero_flag(false);
    }

    // Checking seventh bit value
    if (total & 0b1000_0000) == 0b1000_0000 {
        registers.set_negative_flag(true);
    } else {
        registers.set_negative_flag(false);
    }

    registers.accumulator = total;
}
