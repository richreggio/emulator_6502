// BCC - Branch on Carry Clear

// Operation: Branch on C = 0

// This instruction tests the state of the carry bit and takes a conditional branch if the carry bit is reset.

// It affects no flags or registers other than the program counter and then only if the C flag is not on.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Relative	BCC $nnnn	$90	2	2+t+p

// p: =1 if page is crossed.
// t: =1 if branch is taken.
