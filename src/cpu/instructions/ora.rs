use super::*;

// ORA - "OR" Memory with Accumulator
// Operation: A ∨ M → A
// The ORA instruction transfers the memory and the accumulator to the adder which performs a binary "OR" on a bit-by-bit basis and stores the result in the accumulator.
// This instruction affects the accumulator; sets the zero flag if the result in the accumulator is 0, otherwise resets the zero flag; sets the negative flag if the result in the accumulator has bit 7 on, otherwise resets the negative flag.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode	              | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Immediate                      | ORA #$nn               | $09    | 2	        | 2          |
// | Absolute                       | ORA $nnnn              | $0D    |	3         | 4          |
// | Absolute X-Indexed             | ORA $nnnn,X            | $1D    | 3         | 4+p        |
// | Absolute Y-Indexed	            | ORA $nnnn,Y            | $19    | 3         | 4+p        |
// | Zero Page                      | ORA $nn                | $05    | 2         | 3          |
// | Zero Page X-Indexed            | ORA $nn,X              | $15    | 2         | 4          |
// | Zero Page X-Indexed Indirect   | ORA ($nn,X)            | $01    | 2         | 6          |
// | Zero Page Y-Indexed Indirect   | ORA ($nn),Y            | $11    | 2         | 5+p        |
// | p: =1 if page is crossed       |                        |        |           |            |
// |--------------------------------------------------------------------------------------------

pub fn ora(memory: &mut Memory, registers: &mut Registers, operation: Operation) {
    let tmp_value = match operation.addressing_mode {
        AdMode::Immediate(address) => memory.read_byte(address),
        AdMode::Absolute(address) => memory.read_byte(address),
        AdMode::AbsoluteXIndex(address) => memory.read_byte(address),
        AdMode::AbsoluteYIndex(address) => memory.read_byte(address),
        AdMode::ZeroPage(address) => memory.read_byte(address),
        AdMode::ZeroPageXIndex(address) => memory.read_byte(address),
        AdMode::ZeroPageXIndexIndirect(address) => memory.read_byte(address),
        AdMode::ZeroPageYIndexIndirect(address) => memory.read_byte(address),
        _ => 0,
    };

    let value = tmp_value | registers.accumulator;

    if value == 0 {
        registers.set_zero_flag(true);
    } else {
        registers.set_zero_flag(false);
    }

    // Checking seventh bit value
    if (value & 0b1000_0000) == 0b1000_0000 {
        registers.set_negative_flag(true);
    } else {
        registers.set_negative_flag(false);
    }

    registers.accumulator = value;
}
