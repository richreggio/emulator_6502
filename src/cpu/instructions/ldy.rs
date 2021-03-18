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

pub fn ldy(cpu: &mut CPU, operation: &mut Operation) {
    let value = match operation.addressing_mode {
        AdMode::Immediate(address) => cpu.ram.read_byte(address),
        AdMode::Absolute(address) => cpu.ram.read_byte(address),
        AdMode::AbsoluteXIndex(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPage(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageXIndex(address) => cpu.ram.read_byte(address),
        _ => panic!("Invalid LDY operation"),
    };

    if value == 0 {
        cpu.registers.set_zero_flag(true);
    } else {
        cpu.registers.set_zero_flag(false);
    }

    // Checking seventh bit of value
    if (value & 0b1000_0000) == 0b1000_0000 {
        cpu.registers.set_negative_flag(true);
    } else {
        cpu.registers.set_negative_flag(false);
    }

    cpu.registers.y_register = value;
}
