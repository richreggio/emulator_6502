// TAY - Transfer Accumula Tor To Index Y

// Operation: A → Y

// This instruction moves the value of the accumulator into index register Y without affecting the accumulator.

// TAY instruction only affects the Y register and does not affect either the carry or overflow flags. If the index register Y has bit 7 on, then N is set, otherwise it is reset. If the content of the index register Y equals 0 as a result of the operation, Z is set on, otherwise it is reset.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Implied	TAY 	$A8	1	2
