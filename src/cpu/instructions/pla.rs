// PLA - Pull Accumulator From Stack

// Operation: A↑

// This instruction adds 1 to the current value of the stack pointer and uses it to address the stack and loads the contents of the stack into the A register.

// The PLA instruction does not affect the carry or overflow flags. It sets N if the bit 7 is on in accumulator A as a result of instructions, otherwise it is reset. If accumulator A is zero as a result of the PLA, then the Z flag is set, otherwise it is reset. The PLA instruction changes content of the accumulator A to the contents of the memory location at stack register plus 1 and also increments the stack register.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Implied	PLA 	$68	1	4
