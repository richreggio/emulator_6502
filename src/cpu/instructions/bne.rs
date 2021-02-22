use super::*;

// BNE - Branch on Result Not Zero
// Operation: Branch on Z = 0
// This instruction could also be called "Branch on Not Equal." It tests the Z flag and takes the conditional branch if the Z flag is not on, indicating that the previous result was not zero.
// BNE does not affect any of the flags or registers other than the program counter and only then if the Z flag is reset.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Relative	BNE $nnnn	$D0	2	2+t+p
// p: =1 if page is crossed.
// t: =1 if branch is taken.

pub fn bne(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
