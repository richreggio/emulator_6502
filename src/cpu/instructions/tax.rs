use super::*;

// TAX - Transfer Accumulator To Index X
// Operation: A → X
// This instruction takes the value from accumulator A and trans­ fers or loads it into the index register X without disturbing the content of the accumulator A.
// TAX only affects the index register X, does not affect the carry or overflow flags. The N flag is set if the resultant value in the index register X has bit 7 on, otherwise N is reset. The Z bit is set if the content of the register X is 0 as a result of the opera­tion, otherwise it is reset.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Implied	TAX 	$AA	1	2

pub fn tax(_memory: &mut Memory, registers: &mut Registers, _operation: Operation) {
    registers.x_register = registers.accumulator;
    let value = registers.x_register;

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
}
