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

pub fn las(cpu: &mut CPU, operation: &mut Operation) {
    let tmp_value = match operation.addressing_mode {
        AdMode::AbsoluteYIndex(address) => cpu.ram.read_byte(address),
        _ => panic!("Invalid LAS operation"),
    };

    let value = tmp_value & cpu.registers.stack_pointer;

    cpu.registers.accumulator = value;
    cpu.registers.x_register = value;
    cpu.registers.stack_pointer = value;

    if value == 0 {
        cpu.registers.set_zero_flag(true);
    } else {
        cpu.registers.set_zero_flag(false);
    }

    // Checking seventh bit of value
    if (value & 0b1000_0000) == 0b1000_0000 {
        cpu.registers.set_negative_flag(true);
    } else {
        cpu.registers.set_negative_flag(false);
    }
}
