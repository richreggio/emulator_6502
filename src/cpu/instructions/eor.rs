use super::*;

// EOR - "Exclusive OR" Memory with Accumulator
// Operation: A ⊻ M → A
// The EOR instruction transfers the memory and the accumulator to the adder which performs a binary "EXCLUSIVE OR" on a bit-by-bit basis and stores the result in the accumulator.
// This instruction affects the accumulator; sets the zero flag if the result in the accumulator is 0, otherwise resets the zero flag sets the negative flag if the result in the accumulator has bit 7 on, otherwise resets the negative flag.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Immediate	EOR #$nn	$49	2	2
// Absolute	EOR $nnnn	$4D	3	4
// X-Indexed Absolute	EOR $nnnn,X	$5D	3	4+p
// Y-Indexed Absolute	EOR $nnnn,Y	$59	3	4+p
// Zero Page	EOR $nn	$45	2	3
// X-Indexed Zero Page	EOR $nn,X	$55	2	4
// X-Indexed Zero Page Indirect	EOR ($nn,X)	$41	2	6
// Zero Page Indirect Y-Indexed	EOR ($nn),Y	$51	2	5+p
// p: =1 if page is crossed.

pub fn eor(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
