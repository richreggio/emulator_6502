use super::*;

// ROL - Rotate Left
// Operation: C ← /M7...M0/ ← C
// The rotate left instruction shifts either the accumulator or addressed memory left 1 bit, with the input carry being stored in bit 0 and with the input bit 7 being stored in the carry flags.
// The ROL instruction either shifts the accumulator left 1 bit and stores the carry in accumulator bit 0 or does not affect the internal reg­isters at all. The ROL instruction sets carry equal to the input bit 7, sets N equal to the input bit 6 , sets the Z flag if the result of the ro­ tate is 0, otherwise it resets Z and does not affect the overflow flag at all.
// -----------------------------------------------------------------------------------------------
// | Addressing Mode	              | Assembly Language Form | Opcode   | No. Bytes	| No. Cycles |
// |--------------------------------|------------------------------------------------------------|
// | Accumulator                    | ROL A   	             | $2A      | 1         | 2          |
// | Absolute                       | ROL $nnnn              | $2E      | 3         | 6          |
// | Absolute X-Indexed             | ROL $nnnn,X            | $3E      | 3         | 7          |
// | Zero Page                      | ROL $nn                | $26      | 2         | 5          |
// | Zero Page X-Indexed            | ROL $nn,X              | $36      | 2         | 6          |
// |----------------------------------------------------------------------------------------------

pub fn rol(memory: &mut Memory, registers: &mut Registers, operation: Operation) {
    let value = match operation.addressing_mode {
        AdMode::Accumulator => registers.accumulator,
        AdMode::Absolute(address) => memory.read_byte(address),
        AdMode::AbsoluteXIndex(address) => memory.read_byte(address),
        AdMode::ZeroPage(address) => memory.read_byte(address),
        AdMode::ZeroPageXIndex(address) => memory.read_byte(address),
        _ => 0,
    };

    let mut tmp_value = value << 1;

    if registers.carry_flag_is_set() {
        tmp_value += 1;
    }

    // Seventh bit becomes the carry flag
    if value & 0b1000_0000 == 0b1000_0000 {
        registers.set_carry_flag(true);
    } else {
        registers.set_carry_flag(false);
    }

    let value = tmp_value;

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
        AdMode::Accumulator => registers.accumulator = value,
        AdMode::Absolute(address) => memory.write_byte(address, value),
        AdMode::AbsoluteXIndex(address) => memory.write_byte(address, value),
        AdMode::ZeroPage(address) => memory.write_byte(address, value),
        AdMode::ZeroPageXIndex(address) => memory.write_byte(address, value),
        _ => (),
    };
}
