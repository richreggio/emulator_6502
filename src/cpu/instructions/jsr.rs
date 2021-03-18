use super::*;

// JSR - Jump To Subroutine
// Operation: PC + 2↓, [PC + 1] → PCL, [PC + 2] → PCH
// This instruction transfers control of the program counter to a subroutine location but leaves a return pointer on the stack to allow the user to return to perform the next instruction in the main program after the subroutine is complete. To accomplish this, JSR instruction stores the program counter address which points to the last byte of the jump instruc­tion onto the stack using the stack pointer. The stack byte contains the program count high first, followed by program count low. The JSR then transfers the addresses following the jump instruction to the program counter low and the program counter high, thereby directing the program to begin at that new address.
// The JSR instruction affects no flags, causes the stack pointer to be decremented by 2 and substitutes new values into the program counter low and the program counter high.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Absolute                       | JSR $nnnn              | $20    |	3         | 6          |
// |--------------------------------------------------------------------------------------------

pub fn jsr(cpu: &mut CPU, operation: &mut Operation) {
    let old_program_counter = cpu.registers.program_counter - 1;

    cpu.registers
        .stack_push(&mut cpu.ram, (old_program_counter >> 8) as u8);
    cpu.registers
        .stack_push(&mut cpu.ram, (old_program_counter & 0x00FF) as u8);

    cpu.registers.program_counter = match operation.addressing_mode {
        AdMode::Absolute(address) => address,
        _ => panic!("Invalid JSR operation"),
    }
}
