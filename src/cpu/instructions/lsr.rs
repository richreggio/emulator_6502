// LSR - Logical Shift Right

// Operation: 0 → /M7...M0/ → C

// This instruction shifts either the accumulator or a specified memory location 1 bit to the right, with the higher bit of the result always being set to 0, and the low bit which is shifted out of the field being stored in the carry flag.

// The shift right instruction either affects the accumulator by shift­ing it right 1 or is a read/modify/write instruction which changes a speci­fied memory location but does not affect any internal registers. The shift right does not affect the overflow flag. The N flag is always reset. The Z flag is set if the result of the shift is 0 and reset otherwise. The carry is set equal to bit 0 of the input.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Accumulator	LSR A	$4A	1	2
// Absolute	LSR $nnnn	$4E	3	6
// X-Indexed Absolute	LSR $nnnn,X	$5E	3	7
// Zero Page	LSR $nn	$46	2	5
// X-Indexed Zero Page	LSR $nn,X	$56	2	6
