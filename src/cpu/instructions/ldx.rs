// LDX - Load Index Register X From Memory

// Operation: M → X

// Load the index register X from memory.

// LDX does not affect the C or V flags; sets Z if the value loaded was zero, otherwise resets it; sets N if the value loaded in bit 7 is a 1; otherwise N is reset, and affects only the X register.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Immediate	LDX #$nn	$A2	2	2
// Absolute	LDX $nnnn	$AE	3	4
// Y-Indexed Absolute	LDX $nnnn,Y	$BE	3	4+p
// Zero Page	LDX $nn	$A6	2	3
// Y-Indexed Zero Page	LDX $nn,Y	$B6	2	4

// p: =1 if page is crossed.
