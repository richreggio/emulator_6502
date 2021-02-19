// RTI - Return From Interrupt

// Operation: P↑ PC↑

// This instruction transfers from the stack into the microprocessor the processor status and the program counter location for the instruction which was interrupted. By virtue of the interrupt having stored this data before executing the instruction and thei fact that the RTI reinitializes the microprocessor to the same state as when it was interrupted, the combination of interrupt plus RTI allows truly reentrant coding.

// The RTI instruction reinitializes all flags to the position to the point they were at the time the interrupt was taken and sets the program counter back to its pre-interrupt state. It affects no other registers in the microprocessor.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Implied	RTI 	$40	1	6
