// DEC - Decrement Memory By One

// Operation: M - 1 → M

// This instruction subtracts 1, in two's complement, from the contents of the addressed memory location.

// The decrement instruction does not affect any internal register in the microprocessor. It does not affect the carry or overflow flags. If bit 7 is on as a result of the decrement, then the N flag is set, otherwise it is reset. If the result of the decrement is 0, the Z flag is set, other­wise it is reset.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Absolute	DEC $nnnn	$CE	3	6
// X-Indexed Absolute	DEC $nnnn,X	$DE	3	7
// Zero Page	DEC $nn	$C6	2	5
// X-Indexed Zero Page	DEC $nn,X	$D6	2	6
