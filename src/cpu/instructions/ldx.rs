use super::*;

// LDX - Load Index Register X From Memory
// Operation: M → X
// Load the index register X from memory.
// LDX does not affect the C or V flags; sets Z if the value loaded was zero, otherwise resets it; sets N if the value loaded in bit 7 is a 1; otherwise N is reset, and affects only the X register.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Immediate	LDX #$nn	$A2	2	2
// Absolute	LDX $nnnn	$AE	3	4
// Y-Indexed Absolute	LDX $nnnn,Y	$BE	3	4+p
// Zero Page	LDX $nn	$A6	2	3
// Y-Indexed Zero Page	LDX $nn,Y	$B6	2	4
// p: =1 if page is crossed.

pub fn ldx(memory: &mut Memory, registers: &mut Registers, operation: Operation) {
    let value = match operation.addressing_mode {
        AdMode::Immediate(address) => memory.read_byte(address),
        AdMode::Absolute(address) => memory.read_byte(address),
        AdMode::AbsoluteYIndex(address) => memory.read_byte(address),
        AdMode::ZeroPage(address) => memory.read_byte(address),
        AdMode::ZeroPageYIndex(address) => memory.read_byte(address),
        _ => registers.x_register,
    };

    if value == 0 {
        registers.set_zero_flag(true)
    } else {
        registers.set_zero_flag(false)
    }

    // Checking seventh bit of X Register value
    if (value & 0b1000_0000) == 0b1000_0000 {
        registers.set_negative_flag(true)
    } else {
        registers.set_negative_flag(false)
    }

    registers.x_register = value;
}
