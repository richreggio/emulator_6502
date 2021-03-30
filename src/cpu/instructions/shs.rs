use super::*;

// SHS - Transfer Accumulator "AND" Index Register X to Stack Pointer then Store Stack Pointer "AND" Hi-Byte In Memory
// Operation: A ∧ X → S, S ∧ (H + 1) → M
// The undocumented SHS instruction performs a bit-by-bit AND operation of the value of the accumulator and the value of the index register X and stores the result in the stack pointer. It then performs a bit-by-bit AND operation of the resulting stack pointer and the upper 8 bits of the given address (ignoring the addressing mode's Y offset), plus 1, and transfers the result to the addressed memory location.
// No flags or registers in the microprocessor are affected by the store operation.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Absolute Y-Indexed             | SHS $nnnn,Y            | $9B*   | 3         | 5          |
// |--------------------------------------------------------------------------------------------

pub fn shs(_cpu: &mut Cpu, _operation: &mut Operation) {}
