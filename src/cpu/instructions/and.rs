use super::*;

// AND - "AND" Memory with Accumulator
// Operation: A ∧ M → A
// The AND instruction transfer the accumulator and memory to the adder which performs a bit-by-bit AND operation and stores the result back in the accumulator.
// This instruction affects the accumulator; sets the zero flag if the result in the accumulator is 0, otherwise resets the zero flag; sets the negative flag if the result in the accumulator has bit 7 on, otherwise resets the negative flag.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode	              | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Immediate                      | AND #$nn               | $29    | 2         | 2          |
// | Absolute                       | AND $nnnn              | $2D    | 3         | 4          |
// | Absolute X-Indexed             | AND $nnnn,X            | $3D    | 3         | 4+p        |
// | Absolute Y-Indexed	            | AND $nnnn,Y            | $39    | 3         | 4+p        |
// | Zero Page                      | AND $nn                | $25    | 2         | 3          |
// | Zero Page X-Indexed            | AND $nn,X              | $35    | 2         | 4          |
// | Zero Page X-Indexed Indirect   | AND ($nn,X)            | $21    | 2         | 6          |
// | Zero Page Y-Indexed Indirect   | AND ($nn),Y            | $31    | 2         | 5+p        |
// | p: =1 if page is crossed       |                        |        |           |            |
// |--------------------------------------------------------------------------------------------

pub fn and(cpu: &mut CPU, operation: &mut Operation) {
    let tmp_value = match operation.addressing_mode {
        AdMode::Immediate(address) => cpu.ram.read_byte(address),
        AdMode::Absolute(address) => cpu.ram.read_byte(address),
        AdMode::AbsoluteXIndex(address) => cpu.ram.read_byte(address),
        AdMode::AbsoluteYIndex(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPage(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageXIndex(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageXIndexIndirect(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageYIndexIndirect(address) => cpu.ram.read_byte(address),
        _ => panic!("Invalid AND operation"),
    };

    let value = tmp_value & cpu.registers.accumulator;

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
