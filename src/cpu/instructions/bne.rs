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

pub fn bne(cpu: &mut Cpu, operation: &mut Operation) {
    if !cpu.registers.zero_flag_is_set() {
        operation.cycles += 1;

        let offset = match operation.addressing_mode {
            AdMode::Relative(offset) => offset as u16,
            _ => panic!("Invalid BNE operation"),
        };

        let new_address = offset.wrapping_add(cpu.registers.program_counter as u16) as usize;

        // Page crossed
        if new_address & 0xFF00 != cpu.registers.program_counter & 0xFF00 {
            operation.cycles += 1;
        }

        cpu.registers.program_counter = new_address;
    }
}
