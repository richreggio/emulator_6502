use super::*;

// CPY - Compare Index Register Y To Memory
// Operation: Y - M
// This instruction performs a two's complement subtraction between the index register Y and the specified memory location. The results of the subtraction are not stored anywhere. The instruction is strict­ly used to set the flags.
// CPY affects no registers in the microprocessor and also does not affect the overflow flag. If the value in the index register Y is equal to or greater than the value in the memory, the carry flag will be set, otherwise it will be cleared. If the results of the subtract- tion contain bit 7 on the N bit will be set, otherwise it will be cleared. If the value in the index register Y and the value in the memory are equal, the zero flag will be set, otherwise it will be cleared.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Immediate                      | CPY #$nn               | $C0    | 2 	      | 2          |
// | Absolute                       | CPY $nnnn              | $CC    |	3         | 4          |
// | Zero Page                      | CPY $nn                | $C4    | 2         | 3          |
// |--------------------------------------------------------------------------------------------

pub fn cpy(memory: &mut Memory, registers: &mut Registers, operation: Operation) {
    let tmp_value = match operation.addressing_mode {
        AdMode::Immediate(address) => memory.read_byte(address),
        AdMode::Absolute(address) => memory.read_byte(address),
        AdMode::ZeroPage(address) => memory.read_byte(address),
        _ => 0,
    };

    let (value, carry) = registers.y_register.overflowing_sub(tmp_value);

    if value == 0 {
        registers.set_zero_flag(true);
    } else {
        registers.set_zero_flag(false);
    }

    // Checking seventh bit of value
    if (value & 0b1000_0000) == 0b1000_0000 {
        registers.set_negative_flag(true);
    } else {
        registers.set_negative_flag(false);
    }

    if carry {
        registers.set_carry_flag(true);
    } else {
        registers.set_carry_flag(false);
    }
}
