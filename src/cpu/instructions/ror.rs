// ROR - Rotate Right

// Operation: C → /M7...M0/ → C

// The rotate right instruction shifts either the accumulator or addressed memory right 1 bit with bit 0 shifted into the carry and carry shifted into bit 7.

// The ROR instruction either shifts the accumulator right 1 bit and stores the carry in accumulator bit 7 or does not affect the internal regis­ ters at all. The ROR instruction sets carry equal to input bit 0, sets N equal to the input carry and sets the Z flag if the result of the rotate is 0; otherwise it resets Z and does not affect the overflow flag at all.

// (Available on Microprocessors after June, 1976)
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Accumulator	ROR A	$6A	1	2
// Absolute	ROR $nnnn	$6E	3	6
// X-Indexed Absolute	ROR $nnnn,X	$7E	3	7
// Zero Page	ROR $nn	$66	2	5
// X-Indexed Zero Page	ROR $nn,X	$76	2	6
