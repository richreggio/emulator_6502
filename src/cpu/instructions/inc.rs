use super::*;

// INC - Increment Memory By One
// Operation: M + 1 → M
// This instruction adds 1 to the contents of the addressed memory loca­tion.
// The increment memory instruction does not affect any internal registers and does not affect the carry or overflow flags. If bit 7 is on as the result of the increment,N is set, otherwise it is reset; if the increment causes the result to become 0, the Z flag is set on, otherwise it is reset.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Absolute                       | INC $nnnn              | $EE    |	3         | 6          |
// | Absolute X-Indexed             | INC $nnnn,X            | $FE    | 3         | 7          |
// | Zero Page                      | INC $nn                | $E6    | 2         | 5          |
// | Zero Page X-Indexed            | INC $nn,X              | $F6    | 2         | 6          |
// |--------------------------------------------------------------------------------------------

pub fn inc(memory: &mut Memory, registers: &mut Registers, operation: Operation) {
    let tmp_value = match operation.addressing_mode {
        AdMode::Absolute(address) => memory.read_byte(address),
        AdMode::AbsoluteXIndex(address) => memory.read_byte(address),
        AdMode::ZeroPage(address) => memory.read_byte(address),
        AdMode::ZeroPageXIndex(address) => memory.read_byte(address),
        _ => 0,
    };

    let value = tmp_value.wrapping_add(1);

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
