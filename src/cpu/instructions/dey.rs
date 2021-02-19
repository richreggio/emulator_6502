// DEY - Decrement Index Register Y By One

// Operation: Y - 1 → Y

// This instruction subtracts one from the current value in the in­ dex register Y and stores the result into the index register Y. The result does not affect or consider carry so that the value in the index register Y is decremented to 0 and then through 0 to FF.

// Decrement Y does not affect the carry or overflow flags; if the Y register contains bit 7 on as a result of the decrement the N flag is set, otherwise the N flag is reset. If the Y register is 0 as a result of the decrement, the Z flag is set otherwise the Z flag is reset. This instruction only affects the index register Y.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Implied	DEY 	$88	1	2
