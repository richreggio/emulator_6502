use super::*;

// BPL - Branch on Result Plus
// Operation: Branch on N = 0
// This instruction is the complementary branch to branch on result minus. It is a conditional branch which takes the branch when the N bit is reset (0). BPL is used to test if the previous result bit 7 was off (0) and branch on result minus is used to determine if the previous result was minus or bit 7 was on (1).
// The instruction affects no flags or other registers other than the P counter and only affects the P counter when the N bit is reset.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode	              | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Relative                       | BPL $nnnn              | $10    | 2         | 2+t+p      |
// | p: =1 if page is crossed       |                        |        |           |            |
// | t: =1 if branch is taken       |                        |        |           |            |
// |--------------------------------------------------------------------------------------------

pub fn bpl(cpu: &mut CPU, operation: &mut Operation) {
    if !cpu.registers.negative_flag_is_set() {
        operation.cycles += 1;

        let offset = match operation.addressing_mode {
            AdMode::Relative(offset) => offset as u16,
            _ => panic!("Invalid BPL operation"),
        };

        let new_address = offset.wrapping_add(cpu.registers.program_counter as u16) as usize;

        // Page crossed
        if new_address & 0xFF00 != cpu.registers.program_counter & 0xFF00 {
            operation.cycles += 1;
        }

        cpu.registers.program_counter = new_address;
    }
}
