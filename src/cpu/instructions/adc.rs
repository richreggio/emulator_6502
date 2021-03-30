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

pub fn adc(cpu: &mut Cpu, operation: &mut Operation) {
    let tmp_value = match operation.addressing_mode {
        AdMode::Immediate(address) => cpu.ram.read_byte(address),
        AdMode::Absolute(address) => cpu.ram.read_byte(address),
        AdMode::AbsoluteXIndex(address) => cpu.ram.read_byte(address),
        AdMode::AbsoluteYIndex(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPage(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageXIndex(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageXIndexIndirect(address) => cpu.ram.read_byte(address),
        AdMode::ZeroPageYIndexIndirect(address) => cpu.ram.read_byte(address),
        _ => panic!("Invalid ADC operation"),
    };

    let (mut total, carry) = tmp_value.overflowing_add(cpu.registers.accumulator);

    if cpu.registers.carry_flag_is_set() {
        total += 1;
    }

    if carry {
        cpu.registers.set_carry_flag(true);
    } else {
        cpu.registers.set_carry_flag(false);
    }

    if ((cpu.registers.accumulator & 0b1000_0000) ^ (total & 0b1000_0000))
        & !((cpu.registers.accumulator & 0b1000_0000) ^ (tmp_value & 0b1000_0000))
        == 0b1000_0000
    {
        cpu.registers.set_overflow_flag(true);
    } else {
        cpu.registers.set_overflow_flag(false);
    }

    if total == 0 {
        cpu.registers.set_zero_flag(true);
    } else {
        cpu.registers.set_zero_flag(false);
    }

    // Checking seventh bit value
    if (total & 0b1000_0000) == 0b1000_0000 {
        cpu.registers.set_negative_flag(true);
    } else {
        cpu.registers.set_negative_flag(false);
    }

    cpu.registers.accumulator = total;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn adc_setup(accumulator_value: u8, memory_value: u8) -> (Cpu, Operation) {
        let mut cpu = Cpu::new();
        cpu.registers.initalize(0x8000);

        // Using opcode for Immediate addressing to simplify testing
        cpu.ram.write_byte(0x8000, 0x69);

        // Setting memory value to be added
        cpu.ram.write_byte(0x8001, memory_value);

        // Setting accumulator value
        cpu.registers.accumulator = accumulator_value;

        let operation = Operation::next(&mut cpu);

        (cpu, operation)
    }

    #[test]
    fn adc_integers_no_overflow() {
        let (mut cpu, mut operation) = adc_setup(0x15, 0x20);

        adc(&mut cpu, &mut operation);

        let result = cpu.registers.accumulator;

        assert_eq!(result, 0x35);
        assert!(!cpu.registers.carry_flag_is_set());
    }

    #[test]
    fn adc_integers_with_overflow() {
        let (mut cpu, mut operation) = adc_setup(0xFF, 0x05);

        adc(&mut cpu, &mut operation);

        let result = cpu.registers.accumulator;

        assert_eq!(result, 0x04);
        assert!(cpu.registers.carry_flag_is_set());
    }
}
