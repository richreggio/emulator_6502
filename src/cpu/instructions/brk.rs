use super::*;

// BRK - Break Command
// Operation: PC + 2↓, [FFFE] → PCL, [FFFF] → PCH
// The break command causes the microprocessor to go through an inter­rupt sequence under program control. This means that the program counter of the second byte after the BRK. is automatically stored on the stack along with the processor status at the beginning of the break instruction. The microprocessor then transfers control to the interrupt vector.
// Other than changing the program counter, the break instruction changes no values in either the registers or the flags.
// If an IRQ happens at the same time as a BRK instruction, the BRK instruction is ignored.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Implied	BRK 	$00	1	7

pub fn brk(_memory: &mut Memory, registers: &mut Registers, _operation: Operation) {
    // Using for testing since BRK's opcode is 0x00 and memroy is initalized to all 0's
    println!("Current memory address: {}", registers.program_counter);
}
