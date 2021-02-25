use super::*;

// CLI - Clear Interrupt Disable
// Operation: 0 → I
// This instruction initializes the interrupt disable to a 0. This allows the microprocessor to receive interrupts.
// It affects no registers in the microprocessor and no flags other than the interrupt disable which is cleared.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Implied                        | CLI                    | $58    |	1         | 2          |
// |--------------------------------------------------------------------------------------------

pub fn cli(_memory: &mut Memory, registers: &mut Registers, _operation: Operation) {
    registers.set_decimal_flag(false);
}
