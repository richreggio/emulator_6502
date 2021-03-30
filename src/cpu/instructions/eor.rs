use super::*;

// EOR - "Exclusive OR" Memory with Accumulator
// Operation: A ⊻ M → A
// The EOR instruction transfers the memory and the accumulator to the adder which performs a binary "EXCLUSIVE OR" on a bit-by-bit basis and stores the result in the accumulator.
// This instruction affects the accumulator; sets the zero flag if the result in the accumulator is 0, otherwise resets the zero flag sets the negative flag if the result in the accumulator has bit 7 on, otherwise resets the negative flag.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode	              | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Immediate                      | EOR #$nn               | $49    | 2	        | 2          |
// | Absolute                       | EOR $nnnn              | $4D    |	3         | 4          |
// | Absolute X-Indexed             | EOR $nnnn,X            | $5D    | 3         | 4+p        |
// | Absolute Y-Indexed	            | EOR $nnnn,Y            | $59    | 3         | 4+p        |
// | Zero Page                      | EOR $nn                | $45    | 2         | 3          |
// | Zero Page X-Indexed            | EOR $nn,X              | $55    | 2         | 4          |
// | Zero Page X-Indexed Indirect   | EOR ($nn,X)            | $41    | 2         | 6          |
// | Zero Page Y-Indexed Indirect   | EOR ($nn),Y            | $51    | 2         | 5+p        |
// | p: =1 if page is crossed       |                        |        |           |            |
// |--------------------------------------------------------------------------------------------

pub fn eor(cpu: &mut Cpu, operation: &mut Operation) {
    let tmp_value = match operation.addressing_mode {
        AdMode::Immediate(address) => cpu.ram.read_byte(address),
        AdMode::Absolute(address) => cpu.ram.read_byte(address),
        AdMode::AbsoluteXIndex(address) => cpu.ram.read_byte(address),
        AdMode::AbsoluteYIndex(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPage(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageXIndex(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageXIndexIndirect(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageYIndexIndirect(address) => cpu.ram.read_byte(address),
        _ => panic!("Invalid EOR operation"),
    };

    let value = tmp_value ^ cpu.registers.accumulator;

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
