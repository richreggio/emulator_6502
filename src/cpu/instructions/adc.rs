// ADC - Add Memory to Accumulator with Carry

// Operation: A + M + C → A, C

// This instruction adds the value of memory and carry from the previous operation to the value of the accumulator and stores the result in the accumulator.

// This instruction affects the accumulator; sets the carry flag when the sum of a binary add exceeds 255 or when the sum of a decimal add exceeds 99, otherwise carry is reset. The overflow flag is set when the sign or bit 7 is changed due to the result exceeding +127 or -128, otherwise overflow is reset. The negative flag is set if the accumulator result contains bit 7 on, otherwise the negative flag is reset. The zero flag is set if the accumulator result is 0, otherwise the zero flag is reset.

// Note on the MOS 6502:

// In decimal mode, the N, V and Z flags are not consistent with the decimal result.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Immediate	ADC #$nn	$69	2	2
// Absolute	ADC $nnnn	$6D	3	4
// X-Indexed Absolute	ADC $nnnn,X	$7D	3	4+p
// Y-Indexed Absolute	ADC $nnnn,Y	$79	3	4+p
// Zero Page	ADC $nn	$65	2	3
// X-Indexed Zero Page	ADC $nn,X	$75	2	4
// X-Indexed Zero Page Indirect	ADC ($nn,X)	$61	2	6
// Zero Page Indirect Y-Indexed	ADC ($nn),Y	$71	2	5+p

// p: =1 if page is crossed.
