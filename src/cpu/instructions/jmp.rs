use super::*;

// JMP - JMP Indirect
// Operation: [PC + 1] → PCL, [PC + 2] → PCH
// This instruction establishes a new valne for the program counter.
// It affects only the program counter in the microprocessor and affects no flags in the status register.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Absolute	JMP $nnnn	$4C	3	3
// Absolute Indirect	JMP ($nnnn)	$6C	3	5

pub fn jmp(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
