use super::*;

// PHA - Push Accumulator On Stack
// Operation: A↓
// This instruction transfers the current value of the accumulator to the next location on the stack, automatically decrementing the stack to point to the next empty location.
// The Push A instruction only affects the stack pointer register which is decremented by 1 as a result of the operation. It affects no flags.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Implied	PHA 	$48	1	3

pub fn pha(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
