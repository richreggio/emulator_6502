use super::*;

// ANC - "AND" Memory with Accumulator then Move Negative Flag to Carry Flag
// Operation: A ∧ M → A, N → C
// The undocumented ANC instruction performs a bit-by-bit AND operation of the accumulator and memory and stores the result back in the accumulator.
// This instruction affects the accumulator; sets the zero flag if the result in the accumulator is 0, otherwise resets the zero flag; sets the negative flag and the carry flag if the result in the accumulator has bit 7 on, otherwise resets the negative flag and the carry flag.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Immediate                      | ANC #$nn               | $0B*   | 2         | 2          |
// | Immediate                      | ANC #$nn               | $2B*   | 2         | 2          |
// |--------------------------------------------------------------------------------------------

pub fn anc(cpu: &mut Cpu, operation: &mut Operation) {
    let tmp_value = match operation.addressing_mode {
        AdMode::Immediate(address) => cpu.ram.read_byte(address),
        _ => panic!("Invalid AND operation"),
    };

    let value = cpu.registers.accumulator & tmp_value;

    if value == 0 {
        cpu.registers.set_zero_flag(true);
    } else {
        cpu.registers.set_zero_flag(false);
    }

    // Checking seventh bit value
    if (value & 0b1000_0000) == 0b1000_0000 {
        cpu.registers.set_negative_flag(true);
        cpu.registers.set_carry_flag(true);
    } else {
        cpu.registers.set_negative_flag(false);
        cpu.registers.set_carry_flag(false);
    }

    cpu.registers.accumulator = value;
}
