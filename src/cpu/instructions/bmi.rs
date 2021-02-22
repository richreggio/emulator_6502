use super::*;

// BMI - Branch on Result Minus
// Operation: Branch on N = 1
// This instruction takes the conditional branch if the N bit is set.
// BMI does not affect any of the flags or any other part of the machine other than the program counter and then only if the N bit is on.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Relative	BMI $nnnn	$30	2	2+t+p
// p: =1 if page is crossed.
// t: =1 if branch is taken.

pub fn bmi(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
