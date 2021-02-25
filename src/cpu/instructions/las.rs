use super::*;

// LAS - "AND" Memory with Stack Pointer
// Operation: M ∧ S → A, X, S
// This undocumented instruction performs a bit-by-bit "AND" operation of the stack pointer and memory and stores the result back in the accumulator, the index register X and the stack pointer.
// The LAS instruction does not affect the carry or overflow flags. It sets N if the bit 7 of the result is on, otherwise it is reset. If the result is zero, then the Z flag is set, otherwise it is reset.
// -------------------------------------------------------------------------------------------
// | Addressing Mod               | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |------------------------------|----------------------------------------------------------|
// | Absolute Y-Indexed           | LAS $nnnn,Y	           | $BB*   | 3	        | 4+p        |
// | p: =1 if page is crossed.    |                        |        |           |            |
// |------------------------------------------------------------------------------------------

pub fn las(memory: &mut Memory, registers: &mut Registers, operation: Operation) {
    let tmp_value = match operation.addressing_mode {
        AdMode::AbsoluteYIndex(address) => memory.read_byte(address),
        _ => 0,
    };

    let value = tmp_value & registers.stack_pointer;

    registers.accumulator = value;
    registers.x_register = value;
    registers.stack_pointer = value;

    if value == 0 {
        registers.set_zero_flag(true);
    } else {
        registers.set_zero_flag(false);
    }

    // Checking seventh bit of value
    if (value & 0b1000_0000) == 0b1000_0000 {
        registers.set_negative_flag(true);
    } else {
        registers.set_negative_flag(false);
    }
}
