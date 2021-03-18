use super::*;

// ROR - Rotate Right
// Operation: C → /M7...M0/ → C
// The rotate right instruction shifts either the accumulator or addressed memory right 1 bit with bit 0 shifted into the carry and carry shifted into bit 7.
// The ROR instruction either shifts the accumulator right 1 bit and stores the carry in accumulator bit 7 or does not affect the internal regis­ ters at all. The ROR instruction sets carry equal to input bit 0, sets N equal to the input carry and sets the Z flag if the result of the rotate is 0; otherwise it resets Z and does not affect the overflow flag at all.
// -----------------------------------------------------------------------------------------------
// | Addressing Mode	              | Assembly Language Form | Opcode   | No. Bytes	| No. Cycles |
// |--------------------------------|------------------------------------------------------------|
// | Accumulator                    | ROR A   	             | $6A      | 1         | 2          |
// | Absolute                       | ROR $nnnn              | $6E      | 3         | 6          |
// | Absolute X-Indexed             | ROR $nnnn,X            | $7E      | 3         | 7          |
// | Zero Page                      | ROR $nn                | $66      | 2         | 5          |
// | Zero Page X-Indexed            | ROR $nn,X              | $76      | 2         | 6          |
// |----------------------------------------------------------------------------------------------

pub fn ror(cpu: &mut CPU, operation: &mut Operation) {
    let value = match operation.addressing_mode {
        AdMode::Accumulator => cpu.registers.accumulator,
        AdMode::Absolute(address) => cpu.ram.read_byte(address),
        AdMode::AbsoluteXIndex(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPage(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageXIndex(address) => cpu.ram.read_byte(address),
        _ => panic!("Invalid ROR operation"),
    };

    let mut tmp_value = value >> 1;

    if cpu.registers.carry_flag_is_set() {
        tmp_value += 0b1000_0000;
    }

    // Zeroth bit becomes the carry flag
    if value & 0b0000_0001 == 0b0000_0001 {
        cpu.registers.set_carry_flag(true);
    } else {
        cpu.registers.set_carry_flag(false);
    }

    let value = tmp_value;

    if value == 0 {
        cpu.registers.set_zero_flag(true);
    } else {
        cpu.registers.set_zero_flag(false);
    }

    // Checking seventh bit value
    if (value & 0b1000_0000) == 0b1000_0000 {
        cpu.registers.set_negative_flag(true);
    } else {
        cpu.registers.set_negative_flag(false);
    }

    match operation.addressing_mode {
        AdMode::Accumulator => cpu.registers.accumulator = value,
        AdMode::Absolute(address) => cpu.ram.write_byte(address, value),
        AdMode::AbsoluteXIndex(address) => cpu.ram.write_byte(address, value),
        AdMode::ZeroPage(address) => cpu.ram.write_byte(address, value),
        AdMode::ZeroPageXIndex(address) => cpu.ram.write_byte(address, value),
        _ => panic!("Invalid ROR operation"),
    };
}
