use super::*;

// JAM - Halt the CPU
// Operation: Stop execution
// This undocumented instruction stops execution. The microprocessor will not fetch further instructions, and will neither handle IRQs nor NMIs. It will handle a RESET though.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Implied	JAM 	$02*	1	X
// Implied	JAM 	$12*	1	X
// Implied	JAM 	$22*	1	X
// Implied	JAM 	$32*	1	X
// Implied	JAM 	$42*	1	X
// Implied	JAM 	$52*	1	X
// Implied	JAM 	$62*	1	X
// Implied	JAM 	$72*	1	X
// Implied	JAM 	$92*	1	X
// Implied	JAM 	$B2*	1	X
// Implied	JAM 	$D2*	1	X
// Implied	JAM 	$F2*	1	X

pub fn jam(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
