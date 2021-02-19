// ARR - "AND" Accumulator then Rotate Right

// Operation: (A ∧ M) / 2 → A

// The undocumented ARR instruction performs a bit-by-bit "AND" operation of the accumulator and memory, then shifts the result right 1 bit with bit 0 shifted into the carry and carry shifted into bit 7. It then stores the result back in the accumulator.

// If bit 7 of the result is on, then the N flag is set, otherwise it is reset. The instruction sets the Z flag if the result is 0; otherwise it resets Z.

// The V and C flags depends on the Decimal Mode Flag:

// In decimal mode, the V flag is set if bit 6 is different than the original data's bit 6, otherwise the V flag is reset. The C flag is set if (operand & 0xF0) + (operand & 0x10) is greater than 0x50, otherwise the C flag is reset.

// In binary mode, the V flag is set if bit 6 of the result is different than bit 5 of the result, otherwise the V flag is reset. The C flag is set if the result in the accumulator has bit 6 on, otherwise it is reset.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Immediate	ARR #$nn	$6B*	2	2

// *Undocumented.
