use super::*;

// LDA - Load Accumulator with Memory
// Operation: M → A
// When instruction LDA is executed by the microprocessor, data is transferred from memory to the accumulator and stored in the accumulator.

// LDA affects the contents of the accumulator, does not affect the carry or overflow flags; sets the zero flag if the accumulator is zero as a result of the LDA, otherwise resets the zero flag; sets the negative flag if bit 7 of the accumulator is a 1, other­ wise resets the negative flag.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Immediate	LDA #$nn	$A9	2	2
// Absolute	LDA $nnnn	$AD	3	4
// X-Indexed Absolute	LDA $nnnn,X	$BD	3	4+p
// Y-Indexed Absolute	LDA $nnnn,Y	$B9	3	4+p
// Zero Page	LDA $nn	$A5	2	3
// X-Indexed Zero Page	LDA $nn,X	$B5	2	4
// X-Indexed Zero Page Indirect	LDA ($nn,X)	$A1	2	6
// Zero Page Indirect Y-Indexed	LDA ($nn),Y	$B1	2	5+p
// p: =1 if page is crossed.

pub fn lda(memory: &mut Memory, registers: &mut Registers, operation: Operation) {
    let value = match operation.addressing_mode {
        AddressingMode::Immediate(address) => memory.read_byte(address),
        AddressingMode::Absolute(lo_byte, high_byte) => {
            memory.read_byte_little_endian(lo_byte, high_byte)
        }
        AddressingMode::AbsoluteXIndex(lo_byte, high_byte) => {
            memory.read_byte_little_endian_offset(lo_byte, high_byte, registers.x_register)
        }
        AddressingMode::AbsoluteYIndex(lo_byte, high_byte) => {
            memory.read_byte_little_endian_offset(lo_byte, high_byte, registers.y_register)
        }
        AddressingMode::ZeroPage(address) => memory.read_byte(address),
        AddressingMode::ZeroPageXIndex(address) => {
            memory.read_byte((address + registers.x_register as usize) % 0x0100)
        }
        AddressingMode::ZeroPageXIndexIndirect(address) => {
            let tmp_address =
                memory.read_byte((address + registers.x_register as usize) % 0x0100) as usize;
            memory.read_byte_little_endian(tmp_address, tmp_address + 1)
        }
        AddressingMode::ZeroPageYIndexIndirect(address) => {
            let tmp_address =
                memory.read_byte((address + registers.y_register as usize) % 0x0100) as usize;
            memory.read_byte_little_endian(tmp_address, tmp_address + 1)
        }
        _ => registers.accumulator,
    };
    if value == 0 {
        registers.set_zero_flag(true)
    } else {
        registers.set_zero_flag(false)
    }

    // Checking seventh bit of accumulator value
    if (value & 0b0100_0000) == 0b0100_0000 {
        registers.set_negative_flag(true)
    } else {
        registers.set_negative_flag(false)
    }

    registers.accumulator = value;
}
