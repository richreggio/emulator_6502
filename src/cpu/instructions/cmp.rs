// CMP - Compare Memory and Accumulator

// Operation: A - M

// This instruction subtracts the contents of memory from the contents of the accumulator.

// The use of the CMP affects the following flags: Z flag is set on an equal comparison, reset otherwise; the N flag is set or reset by the result bit 7, the carry flag is set when the value in memory is less than or equal to the accumulator, reset when it is greater than the accumulator. The accumulator is not affected.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Immediate	CMP #$nn	$C9	2	2
// Absolute	CMP $nnnn	$CD	3	4
// X-Indexed Absolute	CMP $nnnn,X	$DD	3	4+p
// Y-Indexed Absolute	CMP $nnnn,Y	$D9	3	4+p
// Zero Page	CMP $nn	$C5	2	3
// X-Indexed Zero Page	CMP $nn,X	$D5	2	4
// X-Indexed Zero Page Indirect	CMP ($nn,X)	$C1	2	6
// Zero Page Indirect Y-Indexed	CMP ($nn),Y	$D1	2	5+p

// p: =1 if page is crossed.
