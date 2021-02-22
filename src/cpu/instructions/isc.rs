use super::*;

// ISC - Increment Memory By One then SBC then Subtract Memory from Accumulator with Borrow
// Operation: M + 1 → M, A - M → A
// This undocumented instruction adds 1 to the contents of the addressed memory loca­tion. It then subtracts the value of the result in memory and borrow from the value of the accumulator, using two's complement arithmetic, and stores the result in the accumulator.
// This instruction affects the accumulator. The carry flag is set if the result is greater than or equal to 0. The carry flag is reset when the result is less than 0, indicating a borrow. The over­flow flag is set when the result exceeds +127 or -127, otherwise it is reset. The negative flag is set if the result in the accumulator has bit 7 on, otherwise it is reset. The Z flag is set if the result in the accumulator is 0, otherwise it is reset.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Absolute	ISC $nnnn	$EF*	3	6
// X-Indexed Absolute	ISC $nnnn,X	$FF*	3	7
// Y-Indexed Absolute	ISC $nnnn,Y	$FB*	3	7
// Zero Page	ISC $nn	$E7*	2	5
// X-Indexed Zero Page	ISC $nn,X	$F7*	2	6
// X-Indexed Zero Page Indirect	ISC ($nn,X)	$E3*	2	8
// Zero Page Indirect Y-Indexed	ISC ($nn),Y	$F3*	2	8

pub fn isc(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
