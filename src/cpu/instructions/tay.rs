use super::*;

// TAY - Transfer Accumula Tor To Index Y
// Operation: A → Y
// This instruction moves the value of the accumulator into index register Y without affecting the accumulator.
// TAY instruction only affects the Y register and does not affect either the carry or overflow flags. If the index register Y has bit 7 on, then N is set, otherwise it is reset. If the content of the index register Y equals 0 as a result of the operation, Z is set on, otherwise it is reset.
// -----------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |--------------------------------|------------------------------------------------------------|
// | Implied                        | TAY                    | $A8      | 1         | 2          |
// |----------------------------------------------------------------------------------------------

pub fn tay(_memory: &mut Memory, registers: &mut Registers, _operation: Operation) {
    registers.y_register = registers.accumulator;
    let value = registers.y_register;

    if value == 0 {
        registers.set_zero_flag(true)
    } else {
        registers.set_zero_flag(false)
    }

    // Checking seventh bit of value
    if (value & 0b1000_0000) == 0b1000_0000 {
        registers.set_negative_flag(true)
    } else {
        registers.set_negative_flag(false)
    }
}
