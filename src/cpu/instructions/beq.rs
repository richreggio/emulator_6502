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

pub fn beq(cpu: &mut Cpu, operation: &mut Operation) {
    if cpu.registers.zero_flag_is_set() {
        operation.cycles += 1;

        let offset = match operation.addressing_mode {
            AdMode::Relative(offset) => offset as u16,
            _ => panic!("Invalid BEQ operation"),
        };

        let new_address = offset.wrapping_add(cpu.registers.program_counter as u16) as usize;

        // Page crossed
        if new_address & 0xFF00 != cpu.registers.program_counter & 0xFF00 {
            operation.cycles += 1;
        }

        cpu.registers.program_counter = new_address;
    }
}
