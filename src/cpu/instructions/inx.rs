use super::*;

// INX - Increment Index Register X By One
// Operation: X + 1 → X
// Increment X adds 1 to the current value of the X register. This is an 8-bit increment which does not affect the carry operation, therefore, if the value of X before the increment was FF, the resulting value is 00.
// INX does not affect the carry or overflow flags; it sets the N flag if the result of the increment has a one in bit 7, otherwise resets N; sets the Z flag if the result of the increment is 0, otherwise it resets the Z flag.
// INX does not affect any other register other than the X register.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Implied	INX 	$E8	1	2

pub fn inx(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
