use super::*;

// CPX - Compare Index Register X To Memory
// Operation: X - M
// This instruction subtracts the value of the addressed memory location from the content of index register X using the adder but does not store the result; therefore, its only use is to set the N, Z and C flags to allow for comparison between the index register X and the value in memory.
// The CPX instruction does not affect any register in the machine; it also does not affect the overflow flag. It causes the carry to be set on if the absolute value of the index register X is equal to or greater than the data from memory. If the value of the memory is greater than the content of the index register X, carry is reset. If the results of the subtraction contain a bit 7, then the N flag is set, if not, it is reset. If the value in memory is equal to the value in index register X, the Z flag is set, otherwise it is reset.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Immediate	CPX #$nn	$E0	2	2
// Absolute	CPX $nnnn	$EC	3	4
// Zero Page	CPX $nn	$E4	2	3

pub fn cpx(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
