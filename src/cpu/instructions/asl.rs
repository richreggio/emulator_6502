// ASL - Arithmetic Shift Left

// Operation: C ← /M7...M0/ ← 0

// The shift left instruction shifts either the accumulator or the address memory location 1 bit to the left, with the bit 0 always being set to 0 and the bit 7 output always being contained in the carry flag. ASL either shifts the accumulator left 1 bit or is a read/modify/write instruction that affects only memory.

// The instruction does not affect the overflow bit, sets N equal to the result bit 7 (bit 6 in the input), sets Z flag if the result is equal to 0, otherwise resets Z and stores the input bit 7 in the carry flag.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Accumulator	ASL A	$0A	1	2
// Absolute	ASL $nnnn	$0E	3	6
// X-Indexed Absolute	ASL $nnnn,X	$1E	3	7
// Zero Page	ASL $nn	$06	2	5
// X-Indexed Zero Page	ASL $nn,X	$16	2	6
