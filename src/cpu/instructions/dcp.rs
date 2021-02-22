use super::*;

// DCP - Decrement Memory By One then Compare with Accumulator
// Operation: M - 1 → M, A - M
// This undocumented instruction subtracts 1, in two's complement, from the contents of the addressed memory location. It then subtracts the contents of memory from the contents of the accumulator.
// The DCP instruction does not affect any internal register in the microprocessor. It does not affect the overflow flag. Z flag is set on an equal comparison, reset otherwise; the N flag is set or reset by the result bit 7, the carry flag is set when the result in memory is less than or equal to the accumulator, reset when it is greater than the accumulator.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Absolute	DCP $nnnn	$CF*	3	6
// X-Indexed Absolute	DCP $nnnn,X	$DF*	3	7
// Y-Indexed Absolute	DCP $nnnn,Y	$DB*	3	7
// Zero Page	DCP $nn	$C7*	2	5
// X-Indexed Zero Page	DCP $nn,X	$D7*	2	6
// X-Indexed Zero Page Indirect	DCP ($nn,X)	$C3*	2	8
// Zero Page Indirect Y-Indexed	DCP ($nn),Y	$D3*	2	8

pub fn dcp(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
