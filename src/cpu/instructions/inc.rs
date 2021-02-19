// INC - Increment Memory By One

// Operation: M + 1 → M

// This instruction adds 1 to the contents of the addressed memory loca­tion.

// The increment memory instruction does not affect any internal registers and does not affect the carry or overflow flags. If bit 7 is on as the result of the increment,N is set, otherwise it is reset; if the increment causes the result to become 0, the Z flag is set on, otherwise it is reset.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Absolute	INC $nnnn	$EE	3	6
// X-Indexed Absolute	INC $nnnn,X	$FE	3	7
// Zero Page	INC $nn	$E6	2	5
// X-Indexed Zero Page	INC $nn,X	$F6	2	6
