use super::*;

// BVC - Branch on Overflow Clear
// Operation: Branch on V = 0
// This instruction tests the status of the V flag and takes the conditional branch if the flag is not set.
// BVC does not affect any of the flags and registers other than the program counter and only when the overflow flag is reset.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode	              | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Relative                       | BVC $nnnn              | $50    | 2         | 2+t+p      |
// | p: =1 if page is crossed       |                        |        |           |            |
// | t: =1 if branch is taken       |                        |        |           |            |
// |--------------------------------------------------------------------------------------------

pub fn bvc(_memory: &mut Memory, registers: &mut Registers, mut operation: Operation) {
    if !registers.overflow_flag_is_set() {
        operation.cycles += 1;

        let offset = match operation.addressing_mode {
            AdMode::Relative(offset) => offset as u16,
            _ => 0,
        };

        let new_address = offset.wrapping_add(registers.program_counter as u16) as usize;

        // Page crossed
        if new_address & 0xFF00 != registers.program_counter & 0xFF00 {
            operation.cycles += 1;
        }

        registers.program_counter = new_address;
    }
}
