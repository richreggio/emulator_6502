use super::*;

// BIT - Test Bits in Memory with Accumulator
// Operation: A ∧ M, M7 → N, M6 → V
// This instruction performs an AND between a memory location and the accumulator but does not store the result of the AND into the accumulator.
// The bit instruction affects the N flag with N being set to the value of bit 7 of the memory being tested, the V flag with V being set equal to bit 6 of the memory being tested and Z being set by the result of the AND operation between the accumulator and the memory if the result is Zero, Z is reset otherwise. It does not affect the accumulator.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode	              | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Absolute                       | BIT $nnnn              | $2C    | 3         | 4          |
// | Zero Page                      | Bit $nn                | $24    | 2         | 3          |
// |--------------------------------------------------------------------------------------------

pub fn bit(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
