use super::*;

// INC - Increment Memory By One
// Operation: M + 1 → M
// This instruction adds 1 to the contents of the addressed memory loca­tion.
// The increment memory instruction does not affect any internal registers and does not affect the carry or overflow flags. If bit 7 is on as the result of the increment,N is set, otherwise it is reset; if the increment causes the result to become 0, the Z flag is set on, otherwise it is reset.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Absolute                       | INC $nnnn              | $EE    |	3         | 6          |
// | Absolute X-Indexed             | INC $nnnn,X            | $FE    | 3         | 7          |
// | Zero Page                      | INC $nn                | $E6    | 2         | 5          |
// | Zero Page X-Indexed            | INC $nn,X              | $F6    | 2         | 6          |
// |--------------------------------------------------------------------------------------------

pub fn inc(cpu: &mut CPU, operation: &mut Operation) {
    let tmp_value = match operation.addressing_mode {
        AdMode::Absolute(address) => cpu.ram.read_byte(address),
        AdMode::AbsoluteXIndex(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPage(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageXIndex(address) => cpu.ram.read_byte(address),
        _ => panic!("Invalid INC operation"),
    };

    let value = tmp_value.wrapping_add(1);

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
        _ => panic!("Invalid INC operation"),
    };
}
