// TXA - Transfer Index X To Accumulator

// Operation: X → A

// This instruction moves the value that is in the index register X to the accumulator A without disturbing the content of the index register X.

// TXA does not affect any register other than the accumula­tor and does not affect the carry or overflow flag. If the result in A has bit 7 on, then the N flag is set, otherwise it is reset. If the resultant value in the accumulator is 0, then the Z flag is set, other­ wise it is reset.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Implied	TXA 	$8A	1	2
