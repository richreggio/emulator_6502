// STY - Store Index Register Y In Memory

// Operation: Y → M

// Transfer the value of the Y register to the addressed memory location.

// STY does not affect any flags or registers in the microprocessor.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Absolute	STY $nnnn	$8C	3	4
// Zero Page	STY $nn	$84	2	3
// X-Indexed Zero Page	STY $nn,X	$94	2	4
