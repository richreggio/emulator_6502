// LAS - "AND" Memory with Stack Pointer

// Operation: M ∧ S → A, X, S

// This undocumented instruction performs a bit-by-bit "AND" operation of the stack pointer and memory and stores the result back in the accumulator, the index register X and the stack pointer.

// The LAS instruction does not affect the carry or overflow flags. It sets N if the bit 7 of the result is on, otherwise it is reset. If the result is zero, then the Z flag is set, otherwise it is reset.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Y-Indexed Absolute	LAS $nnnn,Y	$BB*	3	4+p

// *Undocumented.
// p: =1 if page is crossed.
