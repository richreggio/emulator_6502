// LDA - Load Accumulator with Memory

// Operation: M → A

// When instruction LDA is executed by the microprocessor, data is transferred from memory to the accumulator and stored in the accumulator.

// LDA affects the contents of the accumulator, does not affect the carry or overflow flags; sets the zero flag if the accumulator is zero as a result of the LDA, otherwise resets the zero flag; sets the negative flag if bit 7 of the accumulator is a 1, other­ wise resets the negative flag.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Immediate	LDA #$nn	$A9	2	2
// Absolute	LDA $nnnn	$AD	3	4
// X-Indexed Absolute	LDA $nnnn,X	$BD	3	4+p
// Y-Indexed Absolute	LDA $nnnn,Y	$B9	3	4+p
// Zero Page	LDA $nn	$A5	2	3
// X-Indexed Zero Page	LDA $nn,X	$B5	2	4
// X-Indexed Zero Page Indirect	LDA ($nn,X)	$A1	2	6
// Zero Page Indirect Y-Indexed	LDA ($nn),Y	$B1	2	5+p

// p: =1 if page is crossed.
