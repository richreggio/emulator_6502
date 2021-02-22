use super::*;

// BVC - Branch on Overflow Clear
// Operation: Branch on V = 0
// This instruction tests the status of the V flag and takes the conditional branch if the flag is not set.
// BVC does not affect any of the flags and registers other than the program counter and only when the overflow flag is reset.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Relative	BVC $nnnn	$50	2	2+t+p
// p: =1 if page is crossed.
// t: =1 if branch is taken.

pub fn bvc(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
