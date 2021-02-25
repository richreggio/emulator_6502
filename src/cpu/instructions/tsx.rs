use super::*;

// TSX - Transfer Stack Pointer To Index X
// Operation: S → X
// This instruction transfers the value in the stack pointer to the index register X.
// TSX does not affect the carry or overflow flags. It sets N if bit 7 is on in index X as a result of the instruction, otherwise it is reset. If index X is zero as a result of the TSX, the Z flag is set, other­ wise it is reset. TSX changes the value of index X, making it equal to the content of the stack pointer.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode	            | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Implied                        | TSX                    | $BA    | 1         | 2          |
// |--------------------------------------------------------------------------------------------

pub fn tsx(_memory: &mut Memory, registers: &mut Registers, _operation: Operation) {
    registers.x_register = registers.stack_pointer;
    let value = registers.x_register;

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
}
