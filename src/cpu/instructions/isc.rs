use super::*;

// ISC - Increment Memory By One then SBC then Subtract Memory from Accumulator with Borrow
// Operation: M + 1 → M, A - M → A
// This undocumented instruction adds 1 to the contents of the addressed memory loca­tion. It then subtracts the value of the result in memory and borrow from the value of the accumulator, using two's complement arithmetic, and stores the result in the accumulator.
// This instruction affects the accumulator. The carry flag is set if the result is greater than or equal to 0. The carry flag is reset when the result is less than 0, indicating a borrow. The over­flow flag is set when the result exceeds +127 or -127, otherwise it is reset. The negative flag is set if the result in the accumulator has bit 7 on, otherwise it is reset. The Z flag is set if the result in the accumulator is 0, otherwise it is reset.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Absolute                       | ISC $nnnn              | $EF*   |	3         | 6          |
// | Absolute X-Indexed             | ISC $nnnn,X            | $FF*   | 3         | 7          |
// | Absolute Y-Indexed	            | ISC $nnnn,Y            | $FB*   | 3         | 7          |
// | Zero Page                      | ISC $nn                | $E7*   | 2         | 5          |
// | Zero Page X-Indexed            | ISC $nn,X              | $F7*   | 2         | 6          |
// | Zero Page X-Indexed Indirect   | ISC ($nn,X)            | $E3*   | 2         | 8          |
// | Zero Page Y-Indexed Indirect   | ISC ($nn),Y            | $F3*   | 2         | 8          |
// |--------------------------------------------------------------------------------------------

pub fn isc(_cpu: &mut Cpu, _operation: &mut Operation) {
    // let mut tmp_value = match operation.addressing_mode {
    //     AdMode::Absolute(address) => memory.read_byte(address),
    //     AdMode::AbsoluteXIndex(address) => memory.read_byte(address),
    //     AdMode::AbsoluteYIndex(address) => memory.read_byte(address),
    //     AdMode::ZeroPage(address) => memory.read_byte(address),
    //     AdMode::ZeroPageXIndex(address) => memory.read_byte(address),
    //     AdMode::ZeroPageXIndexIndirect(address) => memory.read_byte(address),
    //     AdMode::ZeroPageYIndexIndirect(address) => memory.read_byte(address),
    //     _ => panic!("Invalid ISC operation"),
    // };

    // tmp_value = tmp_value.wrapping_add(1);

    // let tmp_accumulator = registers.accumulator.wrapping_sub(tmp_value);

    // if tmp_value == tmp_accumulator {
    //     registers.set_zero_flag(true);
    // } else {
    //     registers.set_zero_flag(false);
    // }

    // // Checking seventh bit value
    // if (tmp_value & 0b1000_0000) == 0b1000_0000 {
    //     registers.set_negative_flag(true);
    // } else {
    //     registers.set_negative_flag(false);
    // }

    // if tmp_value <= tmp_accumulator {
    //     registers.set_carry_flag(true);
    // } else {
    //     registers.set_carry_flag(false);
    // }

    // match operation.addressing_mode {
    //     AdMode::Absolute(address) => memory.write_byte(address, tmp_value),
    //     AdMode::AbsoluteXIndex(address) => memory.write_byte(address, tmp_value),
    //     AdMode::AbsoluteYIndex(address) => memory.write_byte(address, tmp_value),
    //     AdMode::ZeroPage(address) => memory.write_byte(address, tmp_value),
    //     AdMode::ZeroPageXIndex(address) => memory.write_byte(address, tmp_value),
    //     AdMode::ZeroPageXIndexIndirect(address) => memory.write_byte(address, tmp_value),
    //     AdMode::ZeroPageYIndexIndirect(address) => memory.write_byte(address, tmp_value),
    //     _ => panic!("Invalid ISC operation"),
    // }
}
