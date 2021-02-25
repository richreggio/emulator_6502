use super::*;

// INY - Increment Index Register Y By One
// Operation: Y + 1 → Y
// Increment Y increments or adds one to the current value in the Y register, storing the result in the Y register. As in the case of INX the primary application is to step thru a set of values using the Y register.
// The INY does not affect the carry or overflow flags, sets the N flag if the result of the increment has a one in bit 7, otherwise resets N, sets Z if as a result of the increment the Y register is zero otherwise resets the Z flag.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Implied                        | INY                    | $C8    |	1         | 2          |
// |--------------------------------------------------------------------------------------------

pub fn iny(_memory: &mut Memory, registers: &mut Registers, _operation: Operation) {
    let value = registers.y_register.wrapping_add(1);

    if value == 0 {
        registers.set_zero_flag(true);
    } else {
        registers.set_zero_flag(false);
    }

    // Checking seventh bit value
    if (value & 0b1000_0000) == 0b1000_0000 {
        registers.set_negative_flag(true);
    } else {
        registers.set_negative_flag(false);
    }

    registers.y_register = value;
}
