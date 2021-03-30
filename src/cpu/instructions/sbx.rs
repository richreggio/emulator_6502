use super::*;

// SBX - Subtract Memory from Accumulator "AND" Index Register X
// Operation: (A ∧ X) - M → X
// This undocumented instruction performs a bit-by-bit "AND" of the value of the accumulator and the index register X and subtracts the value of memory from this result, using two's complement arithmetic, and stores the result in the index register X.
// This instruction affects the index register X. The carry flag is set if the result is greater than or equal to 0. The carry flag is reset when the result is less than 0, indicating a borrow. The negative flag is set if the result in index register X has bit 7 on, otherwise it is reset. The Z flag is set if the result in index register X is 0, otherwise it is reset. The over­flow flag not affected at all.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Immediate	SBX #$nn	$CB*	2	2
// -----------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode   | No. Bytes	| No. Cycles |
// |--------------------------------|------------------------------------------------------------|
// | Immediate                      | SBX #$nn               | $CB*     | 2	        | 2          |
// |----------------------------------------------------------------------------------------------

pub fn sbx(_cpu: &mut Cpu, _operation: &mut Operation) {}
