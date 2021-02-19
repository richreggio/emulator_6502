// ASR - "AND" then Logical Shift Right

// Operation: (A ∧ M) / 2 → A

// The undocumented ASR instruction performs a bit-by-bit AND operation of the accumulator and memory, then shifts the accumulator 1 bit to the right, with the higher bit of the result always being set to 0, and the low bit which is shifted out of the field being stored in the carry flag.

// This instruction affects the accumulator. It does not affect the overflow flag. The N flag is always reset. The Z flag is set if the result of the shift is 0 and reset otherwise. The carry is set equal to bit 0 of the result of the "AND" operation.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Immediate	ASR #$nn	$4B*	2	2

// *Undocumented.
