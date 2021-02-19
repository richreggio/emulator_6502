// CLC - Clear Carry Flag

// Operation: 0 → C

// This instruction initializes the carry flag to a 0. This op­ eration should normally precede an ADC loop. It is also useful when used with a R0L instruction to clear a bit in memory.

// This instruction affects no registers in the microprocessor and no flags other than the carry flag which is reset.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Implied	CLC 	$18	1	2
