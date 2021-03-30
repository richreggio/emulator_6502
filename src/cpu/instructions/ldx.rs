use super::*;

// LDX - Load Index Register X From Memory
// Operation: M → X
// Load the index register X from memory.
// LDX does not affect the C or V flags; sets Z if the value loaded was zero, otherwise resets it; sets N if the value loaded in bit 7 is a 1; otherwise N is reset, and affects only the X register.
// -----------------------------------------------------------------------------------------------
// | Addressing Mode	            | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |--------------------------------|------------------------------------------------------------|
// | Immediate                      | LDX #$nn	             | $A2      | 2         | 2          |
// | Absolute                       | LDX $nnnn              | $AE      | 3         | 4          |
// | Absolute X-Indexed             | LDX $nnnn,X            | $BE      | 3         | 4+p        |
// | Zero Page                      | LDX $nn                | $A6      | 2         | 3          |
// | Zero Page Y-Indexed            | LDX $nn,Y              | $B6      | 2         | 4          |
// | p: =1 if page is crossed       |                        |          |           |            |
// |----------------------------------------------------------------------------------------------

pub fn ldx(cpu: &mut Cpu, operation: &mut Operation) {
    let value = match operation.addressing_mode {
        AdMode::Immediate(address) => cpu.ram.read_byte(address),
        AdMode::Absolute(address) => cpu.ram.read_byte(address),
        AdMode::AbsoluteYIndex(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPage(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageYIndex(address) => cpu.ram.read_byte(address),
        _ => panic!("Invalid LDX operation"),
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

    cpu.registers.x_register = value;
}
