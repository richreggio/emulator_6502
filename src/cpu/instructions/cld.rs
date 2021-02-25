use super::*;

// CLD - Clear Decimal Mode
// Operation: 0 → D
// This instruction sets the decimal mode flag to a 0. This all subsequent ADC and SBC instructions to operate as simple operations.
// CLD affects no registers in the microprocessor and no flags other than the decimal mode flag which is set to a 0.
// The value of the decimal mode flag is indeterminate after a RESET.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Implied                        | CLD                    | $D8    |	1         | 2          |
// |--------------------------------------------------------------------------------------------

pub fn cld(_memory: &mut Memory, registers: &mut Registers, _operation: Operation) {
    registers.set_decimal_flag(false);
}
