use super::*;

// LDA - Load Accumulator with Memory
// Operation: M → A
// When instruction LDA is executed by the microprocessor, data is transferred from memory to the accumulator and stored in the accumulator.
// LDA affects the contents of the accumulator, does not affect the carry or overflow flags; sets the zero flag if the accumulator is zero as a result of the LDA, otherwise resets the zero flag; sets the negative flag if bit 7 of the accumulator is a 1, other­ wise resets the negative flag.
// -----------------------------------------------------------------------------------------------
// | Addressing Mode	            | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |--------------------------------|------------------------------------------------------------|
// | Immediate                      | LDA #$nn               | $A9      | 2	        | 2          |
// | Absolute                       | LDA $nnnn              | $AD	    | 3	        | 4          |
// | Absolute X-Indexed             | LDA $nnnn,X            | $BD      | 3         | 4+p        |
// | Absolute Y-Indexed	            | LDA $nnnn,Y            | $B9      | 3	        | 4+p        |
// | Zero Page                      | LDA $nn                | $A5      | 2	        | 3          |
// | Zero Page X-Indexed            | LDA $nn,X              | $B5	    | 2	        | 4          |
// | Zero Page X-Indexed Indirect   | LDA ($nn,X)            | $A1	    | 2	        | 6          |
// | Zero Page Y-Indexed Indirect   | LDA ($nn),Y            | $B1	    | 2	        | 5+p        |
// | p: =1 if page is crossed       |                        |          |           |            |
// |----------------------------------------------------------------------------------------------

pub fn lda(cpu: &mut CPU, operation: &mut Operation) {
    let value = match operation.addressing_mode {
        AdMode::Immediate(address) => cpu.ram.read_byte(address),
        AdMode::Absolute(address) => cpu.ram.read_byte(address),
        AdMode::AbsoluteXIndex(address) => cpu.ram.read_byte(address),
        AdMode::AbsoluteYIndex(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPage(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageXIndex(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageXIndexIndirect(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageYIndexIndirect(address) => cpu.ram.read_byte(address),
        _ => panic!("Invalid LDA operation"),
    };

    if value == 0 {
        cpu.registers.set_zero_flag(true);
    } else {
        cpu.registers.set_zero_flag(false);
    }

    // Checking seventh bit value
    if (value & 0b1000_0000) == 0b1000_0000 {
        cpu.registers.set_negative_flag(true);
    } else {
        cpu.registers.set_negative_flag(false);
    }

    cpu.registers.accumulator = value;
}
