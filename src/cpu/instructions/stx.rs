// STX - Store Index Register X In Memory

// Operation: X → M

// Transfers value of X register to addressed memory location.

// No flags or registers in the microprocessor are affected by the store operation.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Absolute	STX $nnnn	$8E	3	4
// Zero Page	STX $nn	$86	2	3
// Y-Indexed Zero Page	STX $nn,Y	$96	2	4
