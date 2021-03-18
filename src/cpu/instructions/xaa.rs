use super::*;

// XAA - Non-deterministic Operation of Accumulator, Index Register X, Memory and Bus Contents
// Operation: (A ∨ V) ∧ X ∧ M → A
// The operation of the undocumented XAA instruction depends on the individual microprocessor. On most machines, it performs a bit-by-bit AND operation of the following three operands: The first two are the index register X and memory.
// The third operand is the result of a bit-by-bit AND operation of the accumulator and a magic component. This magic component depends on the individual microprocessor and is usually one of $00, $EE, $EF, $FE and $FF, and may be influenced by the RDY pin, leftover contents of the data bus, the temperature of the microprocessor, the supplied voltage, and other factors.
// On some machines, additional bits of the result may be set or reset depending on non-deterministic factors.
// It then transfers the result to the accumulator.
// XAA does not affect the C or V flags; sets Z if the value loaded was zero, otherwise resets it; sets N if the result in bit 7 is a 1; otherwise N is reset.
// -------------------------------------------------------------------------------------------
// | Addressing Mode	            | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |------------------------------|----------------------------------------------------------|
// | Immediate                    | XAA                    | $8B*   | 2         | 2          |
// |------------------------------------------------------------------------------------------

pub fn xaa(_cpu: &mut CPU, _operation: &mut Operation) {}
