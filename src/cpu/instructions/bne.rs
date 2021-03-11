use super::*;

// BNE - Branch on Result Not Zero
// Operation: Branch on Z = 0
// This instruction could also be called "Branch on Not Equal." It tests the Z flag and takes the conditional branch if the Z flag is not on, indicating that the previous result was not zero.
// BNE does not affect any of the flags or registers other than the program counter and only then if the Z flag is reset.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Relative                       | BNE $nnnn              | $D0    | 2         | 2+t+p      |
// | p: =1 if page is crossed       |                        |        |           |            |
// | t: =1 if branch is taken       |                        |        |           |            |
// |--------------------------------------------------------------------------------------------

pub fn bne(_memory: &mut Memory, registers: &mut Registers, mut operation: Operation) {
    if !registers.zero_flag_is_set() {
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
