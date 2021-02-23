use super::*;

// ASL - Arithmetic Shift Left
// Operation: C ← /M7...M0/ ← 0
// The shift left instruction shifts either the accumulator or the address memory location 1 bit to the left, with the bit 0 always being set to 0 and the bit 7 output always being contained in the carry flag. ASL either shifts the accumulator left 1 bit or is a read/modify/write instruction that affects only memory.
// The instruction does not affect the overflow bit, sets N equal to the result bit 7 (bit 6 in the input), sets Z flag if the result is equal to 0, otherwise resets Z and stores the input bit 7 in the carry flag.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode	              | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Accumulator                    | ASL A                  | $0A    | 1         | 2          |
// | Absolute                       | ASL $nnnn              | $0E    | 3         | 6          |
// | Absolute X-Indexed             | ASL $nnnn,X            | $1E    | 3         | 7          |
// | Zero Page                      | ASL $nn                | $06    | 2         | 5          |
// | Zero Page X-Indexed            | ASL $nn,X              | $16    | 2         | 6          |
// |--------------------------------------------------------------------------------------------

pub fn asl(memory: &mut Memory, registers: &mut Registers, operation: Operation) {
    let mut value = match operation.addressing_mode {
        AdMode::Accumulator => registers.accumulator,
        AdMode::Absolute(address) => memory.read_byte(address),
        AdMode::AbsoluteXIndex(address) => memory.read_byte(address),
        AdMode::ZeroPage(address) => memory.read_byte(address),
        AdMode::ZeroPageXIndex(address) => memory.read_byte(address),
        _ => 0,
    };

    // Seventh bit becomes the carry flag
    if value & 0b1000_0000 == 0b1000_0000 {
        registers.set_carry_flag(true)
    } else {
        registers.set_carry_flag(false)
    }

    value = value << 1;

    if value == 0 {
        registers.set_zero_flag(true)
    } else {
        registers.set_zero_flag(false)
    }

    // Checking seventh bit value
    if (value & 0b1000_0000) == 0b1000_0000 {
        registers.set_negative_flag(true)
    } else {
        registers.set_negative_flag(false)
    }

    match operation.addressing_mode {
        AdMode::Accumulator => registers.accumulator = value,
        AdMode::Absolute(address) => memory.write_byte(address, value),
        AdMode::AbsoluteXIndex(address) => memory.write_byte(address, value),
        AdMode::ZeroPage(address) => memory.write_byte(address, value),
        AdMode::ZeroPageXIndex(address) => memory.write_byte(address, value),
        _ => (),
    };
}
