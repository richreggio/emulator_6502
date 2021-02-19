// DEX - Decrement Index Register X By One

// Operation: X - 1 → X

// This instruction subtracts one from the current value of the index register X and stores the result in the index register X.

// DEX does not affect the carry or overflow flag, it sets the N flag if it has bit 7 on as a result of the decrement, otherwise it resets the N flag; sets the Z flag if X is a 0 as a result of the decrement, otherwise it resets the Z flag.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Implied	DEX 	$CA	1	2
