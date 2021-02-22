use super::*;

// TXS - Transfer Index X To Stack Pointer
// Operation: X → S
// This instruction transfers the value in the index register X to the stack pointer.
// TXS changes only the stack pointer, making it equal to the content of the index register X. It does not affect any of the flags.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Implied	TXS 	$9A	1	2

pub fn txs(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}