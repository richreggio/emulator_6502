use super::*;

// CPY - Compare Index Register Y To Memory
// Operation: Y - M
// This instruction performs a two's complement subtraction between the index register Y and the specified memory location. The results of the subtraction are not stored anywhere. The instruction is strict­ly used to set the flags.
// CPY affects no registers in the microprocessor and also does not affect the overflow flag. If the value in the index register Y is equal to or greater than the value in the memory, the carry flag will be set, otherwise it will be cleared. If the results of the subtract- tion contain bit 7 on the N bit will be set, otherwise it will be cleared. If the value in the index register Y and the value in the memory are equal, the zero flag will be set, otherwise it will be cleared.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Immediate	CPY #$nn	$C0	2	2
// Absolute	CPY $nnnn	$CC	3	4
// Zero Page	CPY $nn	$C4	2	3

pub fn cpy(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
