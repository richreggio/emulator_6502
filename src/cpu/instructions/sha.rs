use super::*;

// SHA - Store Accumulator "AND" Index Register X "AND" Value
// Operation: A ∧ X ∧ V → M
// The undocumented SHA instruction performs a bit-by-bit AND operation of the following three operands: The first two are the accumulator and the index register X.
// The third operand depends on the addressing mode. In the zero page indirect Y-indexed case, the third operand is the data in memory at the given zero page address (ignoring the the addressing mode's Y offset) plus 1. In the Y-indexed absolute case, it is the upper 8 bits of the given address (ignoring the the addressing mode's Y offset), plus 1.
// It then transfers the result to the addressed memory location.
// No flags or registers in the microprocessor are affected by the store operation.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Absolute Y-Indexed             | SHA $nnnn,Y            | $9F*   | 3         | 5          |
// | Zero Page Y-Indexed Indirect   | SHA ($nn),Y            | $93*   | 2         | 6          |
// |--------------------------------------------------------------------------------------------

pub fn sha(_cpu: &mut CPU, _operation: &mut Operation) {}
