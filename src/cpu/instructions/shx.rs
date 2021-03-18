use super::*;

// SHX - Store Index Register X "AND" Value
// Operation: X ∧ (H + 1) → M
// The undocumented SHX instruction performs a bit-by-bit AND operation of the index register X and the upper 8 bits of the given address (ignoring the the addressing mode's Y offset), plus 1. It then transfers the result to the addressed memory location.
// No flags or registers in the microprocessor are affected by the store operation.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Absolute Y-Indexed             | SHX $nnnn,Y            | $9E*   | 3         | 5          |
// |--------------------------------------------------------------------------------------------

pub fn shx(_cpu: &mut CPU, _operation: &mut Operation) {}
