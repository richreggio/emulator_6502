// RLA - Rotate Left then "AND" with Accumulator

// Operation: C ← /M7...M0/ ← C, A ∧ M → A

// The undocumented RLA instruction shifts the addressed memory left 1 bit, with the input carry being stored in bit 0 and with the input bit 7 being stored in the carry flags. It then performs a bit-by-bit AND operation of the result and the value of the accumulator and stores the result back in the accumulator.

// This instruction affects the accumulator; sets the zero flag if the result in the accumulator is 0, otherwise resets the zero flag; sets the negative flag if the result in the accumulator has bit 7 on, otherwise resets the negative flag.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Absolute	RLA $nnnn	$2F*	3	6
// X-Indexed Absolute	RLA $nnnn,X	$3F*	3	7
// Y-Indexed Absolute	RLA $nnnn,Y	$3B*	3	7
// Zero Page	RLA $nn	$27*	2	5
// X-Indexed Zero Page	RLA $nn,X	$37*	2	6
// X-Indexed Zero Page Indirect	RLA ($nn,X)	$23*	2	8
// Zero Page Indirect Y-Indexed	RLA ($nn),Y	$33*	2	8

// *Undocumented.