// ORA - "OR" Memory with Accumulator

// Operation: A ∨ M → A

// The ORA instruction transfers the memory and the accumulator to the adder which performs a binary "OR" on a bit-by-bit basis and stores the result in the accumulator.

// This instruction affects the accumulator; sets the zero flag if the result in the accumulator is 0, otherwise resets the zero flag; sets the negative flag if the result in the accumulator has bit 7 on, otherwise resets the negative flag.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Immediate	ORA #$nn	$09	2	2
// Absolute	ORA $nnnn	$0D	3	4
// X-Indexed Absolute	ORA $nnnn,X	$1D	3	4+p
// Y-Indexed Absolute	ORA $nnnn,Y	$19	3	4+p
// Zero Page	ORA $nn	$05	2	3
// X-Indexed Zero Page	ORA $nn,X	$15	2	4
// X-Indexed Zero Page Indirect	ORA ($nn,X)	$01	2	6
// Zero Page Indirect Y-Indexed	ORA ($nn),Y	$11	2	5+p

// p: =1 if page is crossed.
