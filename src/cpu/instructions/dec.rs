use super::*;

// DEC - Decrement Memory By One
// Operation: M - 1 → M
// This instruction subtracts 1, in two's complement, from the contents of the addressed memory location.
// The decrement instruction does not affect any internal register in the microprocessor. It does not affect the carry or overflow flags. If bit 7 is on as a result of the decrement, then the N flag is set, otherwise it is reset. If the result of the decrement is 0, the Z flag is set, other­wise it is reset.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Absolute                       | DEC $nnnn              | $CE    |	3         | 6          |
// | Absolute X-Indexed             | DEC $nnnn,X            | $DE    | 3         | 7          |
// | Zero Page                      | DEC $nn                | $C6    | 2         | 5          |
// | Zero Page X-Indexed            | DEC $nn,X              | $D6    | 2         | 6          |
// |--------------------------------------------------------------------------------------------

pub fn dec(memory: &mut Memory, registers: &mut Registers, operation: Operation) {
    let tmp_value = match operation.addressing_mode {
        AdMode::Absolute(address) => memory.read_byte(address),
        AdMode::AbsoluteXIndex(address) => memory.read_byte(address),
        AdMode::ZeroPage(address) => memory.read_byte(address),
        AdMode::ZeroPageXIndex(address) => memory.read_byte(address),
        _ => 0,
    };

    let value = tmp_value.wrapping_sub(1);

    if value == 0 {
        registers.set_zero_flag(true);
    } else {
        registers.set_zero_flag(false);
    }

    // Checking seventh bit value
    if (value & 0b1000_0000) == 0b1000_0000 {
        registers.set_negative_flag(true);
    } else {
        registers.set_negative_flag(false);
    }

    match operation.addressing_mode {
        AdMode::Absolute(address) => memory.write_byte(address, value),
        AdMode::AbsoluteXIndex(address) => memory.write_byte(address, value),
        AdMode::ZeroPage(address) => memory.write_byte(address, value),
        AdMode::ZeroPageXIndex(address) => memory.write_byte(address, value),
        _ => (),
    };
}
