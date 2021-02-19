// SAX - Store Accumulator "AND" Index Register X in Memory

// Operation: A ∧ X → M

// The undocumented SAX instruction performs a bit-by-bit AND operation of the value of the accumulator and the value of the index register X and stores the result in memory.

// No flags or registers in the microprocessor are affected by the store operation.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Absolute	SAX $nnnn	$8F*	3	4
// Zero Page	SAX $nn	$87*	2	3
// Y-Indexed Zero Page	SAX $nn,Y	$97*	2	4
// X-Indexed Zero Page Indirect	SAX ($nn,X)	$83*	2	6

// *Undocumented.
