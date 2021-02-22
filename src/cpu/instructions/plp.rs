use super::*;

// PLP - Pull Processor Status From Stack
// Operation: P↑
// This instruction transfers the next value on the stack to the Proces­ sor Status register, thereby changing all of the flags and setting the mode switches to the values from the stack.
// The PLP instruction affects no registers in the processor other than the status register. This instruction could affect all flags in the status register.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Implied	PLP 	$28	1	4

pub fn plp(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
