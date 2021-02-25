use super::*;

// CMP - Compare Memory and Accumulator
// Operation: A - M
// This instruction subtracts the contents of memory from the contents of the accumulator.
// The use of the CMP affects the following flags: Z flag is set on an equal comparison, reset otherwise; the N flag is set or reset by the result bit 7, the carry flag is set when the value in memory is less than or equal to the accumulator, reset when it is greater than the accumulator. The accumulator is not affected.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Immediate                      | CMP #$nn               | $C9    | 2 	      | 2          |
// | Absolute                       | CMP $nnnn              | $CD    |	3         | 4          |
// | Absolute X-Indexed             | CMP $nnnn,X            | $DD    | 3         | 4+p        |
// | Absolute Y-Indexed	            | CMP $nnnn,Y            | $D9    | 3         | 4+p        |
// | Zero Page                      | CMP $nn                | $C5    | 2         | 3          |
// | Zero Page X-Indexed            | CMP $nn,X              | $D5    | 2         | 4          |
// | Zero Page X-Indexed Indirect   | CMP ($nn,X)            | $C1    | 2         | 6          |
// | Zero Page Y-Indexed Indirect   | CMP ($nn),Y            | $D1    | 2         | 5+p        |
// | p: =1 if page is crossed       |                        |        |           |            |
// |--------------------------------------------------------------------------------------------

pub fn cmp(memory: &mut Memory, registers: &mut Registers, operation: Operation) {
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

    let (value, carry) = registers.accumulator.overflowing_sub(tmp_value);

    if value == 0 {
        registers.set_zero_flag(true);
    } else {
        registers.set_zero_flag(false);
    }

    // Checking seventh bit of value
    if (value & 0b1000_0000) == 0b1000_0000 {
        registers.set_negative_flag(true);
    } else {
        registers.set_negative_flag(false);
    }

    if carry {
        registers.set_carry_flag(true);
    } else {
        registers.set_carry_flag(false);
    }
}
