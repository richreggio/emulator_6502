use super::*;

// BVS - Branch on Overflow Set
// Operation: Branch on V = 1
// This instruction tests the V flag and takes the conditional branch if V is on.
// BVS does not affect any flags or registers other than the program, counter and only when the overflow flag is set.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode	              | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Relative                       | BVS $nnnn              | $70    | 2         | 2+t+p      |
// | p: =1 if page is crossed       |                        |        |           |            |
// | t: =1 if branch is taken       |                        |        |           |            |
// |--------------------------------------------------------------------------------------------

pub fn bvs(_memory: &mut Memory, registers: &mut Registers, mut operation: Operation) {
    if registers.overflow_flag_is_set() {
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
