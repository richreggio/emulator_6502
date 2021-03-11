use super::*;

// BEQ - Branch on Result Zero
// Operation: Branch on Z = 1
// This instruction could also be called "Branch on Equal."
// It takes a conditional branch whenever the Z flag is on or the previ­ous result is equal to 0.
// BEQ does not affect any of the flags or registers other than the program counter and only then when the Z flag is set.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode	              | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Relative                       | BEQ $nnnn              | $F0    | 2         | 2+t+p      |
// | p: =1 if page is crossed       |                        |        |           |            |
// | t: =1 if branch is taken       |                        |        |           |            |
// |--------------------------------------------------------------------------------------------

pub fn beq(_memory: &mut Memory, registers: &mut Registers, mut operation: Operation) {
    if registers.zero_flag_is_set() {
        operation.cycles += 1;

        let offset = match operation.addressing_mode {
            AdMode::Relative(offset) => offset,
            _ => 0,
        };

        let new_address = registers.program_counter + offset;

        // Page crossed
        if new_address & 0xFF00 != registers.program_counter & 0xFF00 {
            operation.cycles += 1;
        }

        registers.program_counter = new_address;
    }
}
