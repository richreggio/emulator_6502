use super::*;

// TYA - Transfer Index Y To Accumulator
// Operation: Y → A
// This instruction moves the value that is in the index register Y to accumulator A without disturbing the content of the register Y.
// TYA does not affect any other register other than the accumula­ tor and does not affect the carry or overflow flag. If the result in the accumulator A has bit 7 on, the N flag is set, otherwise it is reset. If the resultant value in the accumulator A is 0, then the Z flag is set, otherwise it is reset.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode	            | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Implied                        | TYA                    | $98    | 1         | 2          |
// |--------------------------------------------------------------------------------------------

pub fn tya(cpu: &mut CPU, _operation: &mut Operation) {
    cpu.registers.stack_pointer = cpu.registers.y_register;
}
