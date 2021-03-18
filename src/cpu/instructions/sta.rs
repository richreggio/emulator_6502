use super::*;

// STA - Store Accumulator in Memory
// Operation: A → M
// This instruction transfers the contents of the accumulator to memory.
// This instruction affects none of the flags in the processor status register and does not affect the accumulator.
// -----------------------------------------------------------------------------------------------
// | Addressing Mode	            | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |--------------------------------|------------------------------------------------------------|
// | Absolute                       | STA $nnnn              | $8D      | 3         | 4          |
// | Absolute X-Indexed             | STA $nnnn,X            | $9D      | 3         | 5          |
// | Absolute Y-Indexed             | STA $nnnn,Y	         | $99      | 3         | 5          |
// | Zero Page                      | STA $nn                | $85      | 2         | 3          |
// | Zero Page X-Indexed            | STA $nn,X              | $95      | 2         | 4          |
// | Zero Page X-Indexed Indirect   | STA ($nn,X)            | $81      | 2         | 6          |
// | Zero Page Y-Indexed Indirect   | STA ($nn),Y            | $91      | 2         | 6          |
// | p: =1 if page is crossed       |                        |          |           |            |
// |----------------------------------------------------------------------------------------------

pub fn sta(cpu: &mut CPU, operation: &mut Operation) {
    let address = match operation.addressing_mode {
        AdMode::Absolute(address) => address,
        AdMode::AbsoluteXIndex(address) => address,
        AdMode::AbsoluteYIndex(address) => address,
        AdMode::ZeroPage(address) => address,
        AdMode::ZeroPageXIndex(address) => address,
        AdMode::ZeroPageXIndexIndirect(address) => address,
        AdMode::ZeroPageYIndexIndirect(address) => address,
        _ => panic!("Invalid STA operation"),
    };
    cpu.ram.write_byte(address, cpu.registers.accumulator);
}
