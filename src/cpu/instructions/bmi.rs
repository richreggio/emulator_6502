use super::*;

// BMI - Branch on Result Minus
// Operation: Branch on N = 1
// This instruction takes the conditional branch if the N bit is set.
// BMI does not affect any of the flags or any other part of the machine other than the program counter and then only if the N bit is on.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode	            | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Relative                       | BMI $nnnn              | $30    | 2         | 2+t+p      |
// | p: =1 if page is crossed       |                        |        |           |            |
// | t: =1 if branch is taken       |                        |        |           |            |
// |--------------------------------------------------------------------------------------------

pub fn bmi(_memory: &mut Memory, registers: &mut Registers, mut operation: Operation) {
    if registers.negative_flag_is_set() {
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
