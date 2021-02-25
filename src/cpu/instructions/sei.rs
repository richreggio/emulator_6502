use super::*;

// SEI - Set Interrupt Disable
// Operation: 1 → I
// This instruction initializes the interrupt disable to a 1. It is used to mask interrupt requests during system reset operations and during interrupt commands.
// It affects no registers in the microprocessor and no flags other than the interrupt disable which is set.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Implied                        | SEI                    | $78    |	1         | 2          |
// |--------------------------------------------------------------------------------------------

pub fn sei(_memory: &mut Memory, registers: &mut Registers, _operation: Operation) {
    registers.set_interrupt_flag(true);
}
