use super::*;

// BCC - Branch on Carry Clear
// Operation: Branch on C = 0
// This instruction tests the state of the carry bit and takes a conditional branch if the carry bit is reset.
// It affects no flags or registers other than the program counter and then only if the C flag is not on.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode	              | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Relative                       | BCC $nnnn              | $90    | 2         | 2+t+p      |
// | p: =1 if page is crossed       |                        |        |           |            |
// | t: =1 if branch is taken       |                        |        |           |            |
// |--------------------------------------------------------------------------------------------

pub fn bcc(_memory: &mut Memory, registers: &mut Registers, mut operation: Operation) {
    if !registers.carry_flag_is_set() {
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
