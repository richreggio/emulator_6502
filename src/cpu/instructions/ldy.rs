use super::*;

// LDY - Load Index Register Y From Memory
// Operation: M → Y
// Load the index register Y from memory.
// LDY does not affect the C or V flags, sets the N flag if the value loaded in bit 7 is a 1, otherwise resets N, sets Z flag if the loaded value is zero otherwise resets Z and only affects the Y register.
// -----------------------------------------------------------------------------------------------
// | Addressing Mode	            | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |--------------------------------|------------------------------------------------------------|
// | Immediate                      | LDY #$nn	             | $A0      | 2         | 2          |
// | Absolute                       | LDY $nnnn              | $AC      | 3         | 4          |
// | Absolute X-Indexed             | LDY $nnnn,X            | $BC      | 3         | 4+p        |
// | Zero Page                      | LDY $nn                | $A4      | 2         | 3          |
// | Zero Page X-Indexed            | LDY $nn,X              | $B4      | 2         | 4          |
// | p: =1 if page is crossed       |                        |          |           |            |
// |----------------------------------------------------------------------------------------------

pub fn ldy(memory: &mut Memory, registers: &mut Registers, operation: Operation) {
    let value = match operation.addressing_mode {
        AdMode::Immediate(address) => memory.read_byte(address),
        AdMode::Absolute(address) => memory.read_byte(address),
        AdMode::AbsoluteXIndex(address) => memory.read_byte(address),
        AdMode::ZeroPage(address) => memory.read_byte(address),
        AdMode::ZeroPageXIndex(address) => memory.read_byte(address),
        _ => registers.y_register,
    };

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

    registers.y_register = value;
}
