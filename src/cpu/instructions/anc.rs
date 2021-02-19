// ANC - "AND" Memory with Accumulator then Move Negative Flag to Carry Flag

// Operation: A ∧ M → A, N → C

// The undocumented ANC instruction performs a bit-by-bit AND operation of the accumulator and memory and stores the result back in the accumulator.

// This instruction affects the accumulator; sets the zero flag if the result in the accumulator is 0, otherwise resets the zero flag; sets the negative flag and the carry flag if the result in the accumulator has bit 7 on, otherwise resets the negative flag and the carry flag.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Immediate	ANC #$nn	$0B*	2	2
// Immediate	ANC #$nn	$2B*	2	2

// *Undocumented.