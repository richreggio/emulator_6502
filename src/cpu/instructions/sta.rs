// STA - Store Accumulator in Memory

// Operation: A → M

// This instruction transfers the contents of the accumulator to memory.

// This instruction affects none of the flags in the processor status register and does not affect the accumulator.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Absolute	STA $nnnn	$8D	3	4
// X-Indexed Absolute	STA $nnnn,X	$9D	3	5
// Y-Indexed Absolute	STA $nnnn,Y	$99	3	5
// Zero Page	STA $nn	$85	2	3
// X-Indexed Zero Page	STA $nn,X	$95	2	4
// X-Indexed Zero Page Indirect	STA ($nn,X)	$81	2	6
// Zero Page Indirect Y-Indexed	STA ($nn),Y	$91	2	6
