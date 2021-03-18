use super::*;

// SHY - Store Index Register Y "AND" Value
// Operation: Y ∧ (H + 1) → M
// The undocumented SHY instruction performs a bit-by-bit AND operation of the index register Y and the upper 8 bits of the given address (ignoring the the addressing mode's X offset), plus 1. It then transfers the result to the addressed memory location.
// No flags or registers in the microprocessor are affected by the store operation.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Absolute X-Indexed             | SHY $nnnn,X            | $9C*   | 3         | 5          |
// |--------------------------------------------------------------------------------------------

pub fn shy(_cpu: &mut CPU, _operation: &mut Operation) {}
