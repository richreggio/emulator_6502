use super::*;

// SED - Set Decimal Mode
// Operation: 1 → D
// This instruction sets the decimal mode flag D to a 1. This makes all subsequent ADC and SBC instructions operate as a decimal arithmetic operation.
// SED affects no registers in the microprocessor and no flags other than the decimal mode which is set to a 1.
// The value of the decimal mode flag is indeterminate after a RESET.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Implied                        | SED                    | $F8    |	1         | 2          |
// |--------------------------------------------------------------------------------------------

pub fn sed(cpu: &mut Cpu, _operation: &mut Operation) {
    cpu.registers.set_decimal_flag(true);
}
