use super::*;

// SBC - Subtract Memory from Accumulator with Borrow
// Operation: A - M - ~C → A
// This instruction subtracts the value of memory and borrow from the value of the accumulator, using two's complement arithmetic, and stores the result in the accumulator. Borrow is defined as the carry flag complemented; therefore, a resultant carry flag indicates that a borrow has not occurred.
// This instruction affects the accumulator. The carry flag is set if the result is greater than or equal to 0. The carry flag is reset when the result is less than 0, indicating a borrow. The over­flow flag is set when the result exceeds +127 or -127, otherwise it is reset. The negative flag is set if the result in the accumulator has bit 7 on, otherwise it is reset. The Z flag is set if the result in the accumulator is 0, otherwise it is reset.
// In decimal mode, the N, V and Z flags are not consistent with the decimal result.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Immediate	SBC #$nn	$E9	2	2
// Immediate	SBC #$nn	$EB*	2	2
// Absolute	SBC $nnnn	$ED	3	4
// X-Indexed Absolute	SBC $nnnn,X	$FD	3	4+p
// Y-Indexed Absolute	SBC $nnnn,Y	$F9	3	4+p
// Zero Page	SBC $nn	$E5	2	3
// X-Indexed Zero Page	SBC $nn,X	$F5	2	4
// X-Indexed Zero Page Indirect	SBC ($nn,X)	$E1	2	6
// Zero Page Indirect Y-Indexed	SBC ($nn),Y	$F1	2	5+p
// p: =1 if page is crossed.

pub fn sbc(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
