use super::*;

// DEC - Decrement Memory By One
// Operation: M - 1 → M
// This instruction subtracts 1, in two's complement, from the contents of the addressed memory location.
// The decrement instruction does not affect any internal register in the microprocessor. It does not affect the carry or overflow flags. If bit 7 is on as a result of the decrement, then the N flag is set, otherwise it is reset. If the result of the decrement is 0, the Z flag is set, other­wise it is reset.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Absolute                       | DEC $nnnn              | $CE    |	3         | 6          |
// | Absolute X-Indexed             | DEC $nnnn,X            | $DE    | 3         | 7          |
// | Zero Page                      | DEC $nn                | $C6    | 2         | 5          |
// | Zero Page X-Indexed            | DEC $nn,X              | $D6    | 2         | 6          |
// |--------------------------------------------------------------------------------------------

pub fn dec(cpu: &mut Cpu, operation: &mut Operation) {
    let tmp_value = match operation.addressing_mode {
        AdMode::Absolute(address) => cpu.ram.read_byte(address),
        AdMode::AbsoluteXIndex(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPage(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageXIndex(address) => cpu.ram.read_byte(address),
        _ => panic!("Invalid DEC operation"),
    };

    let value = tmp_value.wrapping_sub(1);

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

    match operation.addressing_mode {
        AdMode::Absolute(address) => cpu.ram.write_byte(address, value),
        AdMode::AbsoluteXIndex(address) => cpu.ram.write_byte(address, value),
        AdMode::ZeroPage(address) => cpu.ram.write_byte(address, value),
        AdMode::ZeroPageXIndex(address) => cpu.ram.write_byte(address, value),
        _ => panic!("Invalid DEC operation"),
    };
}
