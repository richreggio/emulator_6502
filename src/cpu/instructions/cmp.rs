use super::*;

// CMP - Compare Memory and Accumulator
// Operation: A - M
// This instruction subtracts the contents of memory from the contents of the accumulator.
// The use of the CMP affects the following flags: Z flag is set on an equal comparison, reset otherwise; the N flag is set or reset by the result bit 7, the carry flag is set when the value in memory is less than or equal to the accumulator, reset when it is greater than the accumulator. The accumulator is not affected.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Immediate                      | CMP #$nn               | $C9    | 2 	      | 2          |
// | Absolute                       | CMP $nnnn              | $CD    |	3         | 4          |
// | Absolute X-Indexed             | CMP $nnnn,X            | $DD    | 3         | 4+p        |
// | Absolute Y-Indexed	            | CMP $nnnn,Y            | $D9    | 3         | 4+p        |
// | Zero Page                      | CMP $nn                | $C5    | 2         | 3          |
// | Zero Page X-Indexed            | CMP $nn,X              | $D5    | 2         | 4          |
// | Zero Page X-Indexed Indirect   | CMP ($nn,X)            | $C1    | 2         | 6          |
// | Zero Page Y-Indexed Indirect   | CMP ($nn),Y            | $D1    | 2         | 5+p        |
// | p: =1 if page is crossed       |                        |        |           |            |
// |--------------------------------------------------------------------------------------------

pub fn cmp(cpu: &mut Cpu, operation: &mut Operation) {
    let tmp_value = match operation.addressing_mode {
        AdMode::Immediate(address) => cpu.ram.read_byte(address),
        AdMode::Absolute(address) => cpu.ram.read_byte(address),
        AdMode::AbsoluteXIndex(address) => cpu.ram.read_byte(address),
        AdMode::AbsoluteYIndex(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPage(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageXIndex(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageXIndexIndirect(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageYIndexIndirect(address) => cpu.ram.read_byte(address),
        _ => panic!("Invalid CMP operation"),
    };

    let (value, carry) = cpu.registers.accumulator.overflowing_sub(tmp_value);

    if value == 0 {
        cpu.registers.set_zero_flag(true);
    } else {
        cpu.registers.set_zero_flag(false);
    }

    // Checking seventh bit of value
    if (value & 0b1000_0000) == 0b1000_0000 {
        cpu.registers.set_negative_flag(true);
    } else {
        cpu.registers.set_negative_flag(false);
    }

    if carry {
        cpu.registers.set_carry_flag(true);
    } else {
        cpu.registers.set_carry_flag(false);
    }
}
