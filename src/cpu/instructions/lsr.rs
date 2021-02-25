use super::*;

// LSR - Logical Shift Right
// Operation: 0 → /M7...M0/ → C
// This instruction shifts either the accumulator or a specified memory location 1 bit to the right, with the higher bit of the result always being set to 0, and the low bit which is shifted out of the field being stored in the carry flag.
// The shift right instruction either affects the accumulator by shift­ing it right 1 or is a read/modify/write instruction which changes a speci­fied memory location but does not affect any internal registers. The shift right does not affect the overflow flag. The N flag is always reset. The Z flag is set if the result of the shift is 0 and reset otherwise. The carry is set equal to bit 0 of the input.
// -----------------------------------------------------------------------------------------------
// | Addressing Mode	              | Assembly Language Form | Opcode   | No. Bytes	| No. Cycles |
// |--------------------------------|------------------------------------------------------------|
// | Accumulator                    | LSR A   	             | $4A      | 1         | 2          |
// | Absolute                       | LSR $nnnn              | $4E      | 3         | 6          |
// | Absolute X-Indexed             | LSR $nnnn,X            | $5E      | 3         | 7          |
// | Zero Page                      | LSR $nn                | $46      | 2         | 5          |
// | Zero Page X-Indexed            | LSR $nn,X              | $56      | 2         | 6          |
// | p: =1 if page is crossed       |                        |          |           |            |
// |----------------------------------------------------------------------------------------------

pub fn lsr(memory: &mut Memory, registers: &mut Registers, operation: Operation) {
    let tmp_value = match operation.addressing_mode {
        AdMode::Accumulator => registers.accumulator,
        AdMode::Absolute(address) => memory.read_byte(address),
        AdMode::AbsoluteXIndex(address) => memory.read_byte(address),
        AdMode::ZeroPage(address) => memory.read_byte(address),
        AdMode::ZeroPageXIndex(address) => memory.read_byte(address),
        _ => 0,
    };

    // Zeroth bit becomes the carry flag
    if tmp_value & 0b0000_0001 == 0b0000_0001 {
        registers.set_carry_flag(true);
    } else {
        registers.set_carry_flag(false);
    }

    let value = tmp_value >> 1;

    if value == 0 {
        registers.set_zero_flag(true);
    } else {
        registers.set_zero_flag(false);
    }

    registers.set_negative_flag(false);

    match operation.addressing_mode {
        AdMode::Accumulator => registers.accumulator = value,
        AdMode::Absolute(address) => memory.write_byte(address, value),
        AdMode::AbsoluteXIndex(address) => memory.write_byte(address, value),
        AdMode::ZeroPage(address) => memory.write_byte(address, value),
        AdMode::ZeroPageXIndex(address) => memory.write_byte(address, value),
        _ => (),
    };
}
