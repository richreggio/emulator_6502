use super::*;

// PHP - Push Processor Status On Stack
// Operation: P↓
// This instruction transfers the contents of the processor status reg­ ister unchanged to the stack, as governed by the stack pointer.
// The PHP instruction affects no registers or flags in the micropro­cessor.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode	            | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Implied                        | PHP                    | $08    | 1         | 3          |
// |--------------------------------------------------------------------------------------------

pub fn php(memory: &mut Memory, registers: &mut Registers, _operation: Operation) {
    registers.stack_push(memory, registers.get_processor_status());
}
