use super::*;

// DCP - Decrement Memory By One then Compare with Accumulator
// Operation: M - 1 → M, A - M
// This undocumented instruction subtracts 1, in two's complement, from the contents of the addressed memory location. It then subtracts the contents of memory from the contents of the accumulator.
// The DCP instruction does not affect any internal register in the microprocessor. It does not affect the overflow flag. Z flag is set on an equal comparison, reset otherwise; the N flag is set or reset by the result bit 7, the carry flag is set when the result in memory is less than or equal to the accumulator, reset when it is greater than the accumulator.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Absolute                       | DCP $nnnn              | $CF*   |	3         | 6          |
// | Absolute X-Indexed             | DCP $nnnn,X            | $DF*   | 3         | 7          |
// | Absolute Y-Indexed	            | DCP $nnnn,Y            | $DB*   | 3         | 7          |
// | Zero Page                      | DCP $nn                | $C7*   | 2         | 5          |
// | Zero Page X-Indexed            | DCP $nn,X              | $D7*   | 2         | 6          |
// | Zero Page X-Indexed Indirect   | DCP ($nn,X)            | $C3*   | 2         | 8          |
// | Zero Page Y-Indexed Indirect   | DCP ($nn),Y            | $D3*   | 2         | 8          |
// |--------------------------------------------------------------------------------------------

pub fn dcp(cpu: &mut Cpu, operation: &mut Operation) {
    let mut tmp_value = match operation.addressing_mode {
        AdMode::Absolute(address) => cpu.ram.read_byte(address),
        AdMode::AbsoluteXIndex(address) => cpu.ram.read_byte(address),
        AdMode::AbsoluteYIndex(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPage(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageXIndex(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageXIndexIndirect(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageYIndexIndirect(address) => cpu.ram.read_byte(address),
        _ => panic!("Invalid DCP operation"),
    };

    tmp_value = tmp_value.wrapping_sub(1);

    let tmp_accumulator = cpu.registers.accumulator.wrapping_sub(tmp_value);

    if tmp_value == tmp_accumulator {
        cpu.registers.set_zero_flag(true);
    } else {
        cpu.registers.set_zero_flag(false);
    }

    // Checking seventh bit value
    if (tmp_value & 0b1000_0000) == 0b1000_0000 {
        cpu.registers.set_negative_flag(true);
    } else {
        cpu.registers.set_negative_flag(false);
    }

    if tmp_value <= tmp_accumulator {
        cpu.registers.set_carry_flag(true);
    } else {
        cpu.registers.set_carry_flag(false);
    }

    match operation.addressing_mode {
        AdMode::Absolute(address) => cpu.ram.write_byte(address, tmp_value),
        AdMode::AbsoluteXIndex(address) => cpu.ram.write_byte(address, tmp_value),
        AdMode::AbsoluteYIndex(address) => cpu.ram.write_byte(address, tmp_value),
        AdMode::ZeroPage(address) => cpu.ram.write_byte(address, tmp_value),
        AdMode::ZeroPageXIndex(address) => cpu.ram.write_byte(address, tmp_value),
        AdMode::ZeroPageXIndexIndirect(address) => cpu.ram.write_byte(address, tmp_value),
        AdMode::ZeroPageYIndexIndirect(address) => cpu.ram.write_byte(address, tmp_value),
        _ => panic!("Invalid DCP operation"),
    }
}
