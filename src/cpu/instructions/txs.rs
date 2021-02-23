use super::*;

// TXS - Transfer Index X To Stack Pointer
// Operation: X → S
// This instruction transfers the value in the index register X to the stack pointer.
// TXS changes only the stack pointer, making it equal to the content of the index register X. It does not affect any of the flags.
// -----------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |--------------------------------|------------------------------------------------------------|
// | Implied                        | TXS                    | $08      | 1         | 3          |
// |----------------------------------------------------------------------------------------------

pub fn txs(_memory: &mut Memory, registers: &mut Registers, _operation: Operation) {
    registers.stack_pointer = registers.x_register;
}
