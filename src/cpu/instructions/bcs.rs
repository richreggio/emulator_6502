use super::*;

// BCS - Branch on Carry Set
// Operation: Branch on C = 1
// This instruction takes the conditional branch if the carry flag is on.
// BCS does not affect any of the flags or registers except for the program counter and only then if the carry flag is on.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Relative	BCS $nnnn	$B0	2	2+t+p
// p: =1 if page is crossed.
// t: =1 if branch is taken.

pub fn bcs(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
