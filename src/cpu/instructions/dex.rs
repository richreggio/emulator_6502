use super::*;

// DEX - Decrement Index Register X By One
// Operation: X - 1 → X
// This instruction subtracts one from the current value of the index register X and stores the result in the index register X.
// DEX does not affect the carry or overflow flag, it sets the N flag if it has bit 7 on as a result of the decrement, otherwise it resets the N flag; sets the Z flag if X is a 0 as a result of the decrement, otherwise it resets the Z flag.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Implied                        | DEX                    | $CA    |	1         | 2          |
// |--------------------------------------------------------------------------------------------

pub fn dex(_memory: &mut Memory, registers: &mut Registers, _operation: Operation) {
    let value = registers.x_register.wrapping_sub(1);

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

    registers.x_register = value;
}
