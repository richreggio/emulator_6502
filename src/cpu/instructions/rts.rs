use super::*;

// RTS - Return From Subroutme
// Operation: PC↑, PC + 1 → PC
// This instruction loads the program count low and program count high from the stack into the program counter and increments the program counter so that it points to the instruction following the JSR. The stack pointer is adjusted by incrementing it twice.
// The RTS instruction does not affect any flags and affects only PCL and PCH.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Implied	RTS 	$60	1	6

pub fn rts(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
