use super::*;

// ADC - Add Memory to Accumulator with Carry
// Operation: A + M + C → A, C
// This instruction adds the value of memory and carry from the previous operation to the value of the accumulator and stores the result in the accumulator.
// This instruction affects the accumulator; sets the carry flag when the sum of a binary add exceeds 255 or when the sum of a decimal add exceeds 99, otherwise carry is reset. The overflow flag is set when the sign or bit 7 is changed due to the result exceeding +127 or -128, otherwise overflow is reset. The negative flag is set if the accumulator result contains bit 7 on, otherwise the negative flag is reset. The zero flag is set if the accumulator result is 0, otherwise the zero flag is reset.
// In decimal mode, the N, V and Z flags are not consistent with the decimal result.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode	            | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Immediate                      | ADC #$nn               | $69    | 2	      | 2          |
// | Absolute                       | ADC $nnnn              | $6D    |	3         | 4          |
// | Absolute X-Indexed             | ADC $nnnn,X            | $7D    | 3         | 4+p        |
// | Absolute Y-Indexed	            | ADC $nnnn,Y            | $79    | 3         | 4+p        |
// | Zero Page                      | ADC $nn                | $65    | 2         | 3          |
// | Zero Page X-Indexed            | ADC $nn,X              | $75    | 2         | 4          |
// | Zero Page X-Indexed Indirect   | ADC ($nn,X)            | $61    | 2         | 6          |
// | Zero Page Y-Indexed Indirect   | ADC ($nn),Y            | $71    | 2         | 5+p        |
// | p: =1 if page is crossed       |                        |        |           |            |
// |--------------------------------------------------------------------------------------------

pub fn adc(memory: &mut Memory, registers: &mut Registers, operation: Operation) {
    let value = match operation.addressing_mode {
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

    let (mut total, carry) = value.overflowing_add(registers.accumulator);

    if registers.carry_flag_is_set() {
        total += 1;
    }

    if carry {
        registers.set_carry_flag(true);
    } else {
        registers.set_carry_flag(false);
    }

    // If the total's 7th bit and the accumulators 7th bit are different
    // then result exceeded +127 or -128 so set overflow flag
    if (total & 0b1000_0000) ^ (registers.accumulator & 0b1000_0000) == 0b1000_0000 {
        registers.set_overflow_flag(true);
    } else {
        registers.set_overflow_flag(false);
    }

    if total == 0 {
        registers.set_zero_flag(true);
    } else {
        registers.set_zero_flag(false);
    }

    // Checking seventh bit value
    if (total & 0b1000_0000) == 0b1000_0000 {
        registers.set_negative_flag(true);
    } else {
        registers.set_negative_flag(false);
    }

    registers.accumulator = total;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn adc_setup(accumulator_value: u8, memory_value: u8) -> (Memory, Registers, Operation) {
        let mut memory = Memory::new_initalized();
        let mut registers = Registers::new();
        registers.initalize(0x8000);

        // Using opcode for Immediate addressing to simplify testing
        memory.write_byte(0x8000, 0x69);

        // Setting memory value to be added
        memory.write_byte(0x8001, memory_value);

        // Setting accumulator value
        registers.accumulator = accumulator_value;

        let operation = Operation::next(&mut registers, &memory);

        (memory, registers, operation)
    }

    #[test]
    fn adc_integers_no_overflow() {
        let (mut memory, mut registers, operation) = adc_setup(0x15, 0x20);

        adc(&mut memory, &mut registers, operation);

        let result = registers.accumulator;

        assert_eq!(result, 0x35);
        assert!(!registers.carry_flag_is_set());
    }

    #[test]
    fn adc_integers_with_overflow() {
        let (mut memory, mut registers, operation) = adc_setup(0xFF, 0x05);

        adc(&mut memory, &mut registers, operation);

        let result = registers.accumulator;

        assert_eq!(result, 0x04);
        assert!(registers.carry_flag_is_set());
    }
}
