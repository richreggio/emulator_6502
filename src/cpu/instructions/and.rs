use super::*;

// AND - "AND" Memory with Accumulator
// Operation: A ∧ M → A
// The AND instruction transfer the accumulator and memory to the adder which performs a bit-by-bit AND operation and stores the result back in the accumulator.
// This instruction affects the accumulator; sets the zero flag if the result in the accumulator is 0, otherwise resets the zero flag; sets the negative flag if the result in the accumulator has bit 7 on, otherwise resets the negative flag.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Immediate	AND #$nn	$29	2	2
// Absolute	AND $nnnn	$2D	3	4
// X-Indexed Absolute	AND $nnnn,X	$3D	3	4+p
// Y-Indexed Absolute	AND $nnnn,Y	$39	3	4+p
// Zero Page	AND $nn	$25	2	3
// X-Indexed Zero Page	AND $nn,X	$35	2	4
// X-Indexed Zero Page Indirect	AND ($nn,X)	$21	2	6
// Zero Page Indirect Y-Indexed	AND ($nn),Y	$31	2	5+p
// p: =1 if page is crossed.

pub fn and(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
