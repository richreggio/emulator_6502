use crate::cpu::Cpu;

#[derive(Debug, PartialEq)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate(usize),
    Absolute(usize),
    AbsoluteXIndex(usize),
    AbsoluteYIndex(usize),
    AbsoluteIndirect(usize),
    ZeroPage(usize),
    ZeroPageXIndex(usize),
    ZeroPageYIndex(usize),
    ZeroPageXIndexIndirect(usize),
    ZeroPageYIndexIndirect(usize),
    Relative(usize),
}

impl AddressingMode {
    pub fn process(&self, cpu: &Cpu) -> (AddressingMode, u8) {
        match *self {
            // Implied
            // In the implied addressing mode, the address containing the operand is implicitly stated in the operation code of the instruction.
            AddressingMode::Implied => (AddressingMode::Implied, 0),
            // Accumulator
            // This form of addressing is represented with a one byte instruction, implying an operation on the accumulator.
            AddressingMode::Accumulator => (AddressingMode::Accumulator, 0),
            // Immediate #$nn
            // In immediate addressing, the operand is contained in the second byte of the instruction, with no further memory addressing required.
            AddressingMode::Immediate(second_byte) => (AddressingMode::Immediate(second_byte), 0),
            // Absolute $nnnn
            // In absolute addressing, the second byte of the instruction specifies the eight low order bits of the effective address while the third byte specifies the eight high order bits. Thus, the absolute addressing mode allows access to the entire 65 K bytes of addressable memory.
            AddressingMode::Absolute(second_byte) => {
                let lo_byte = cpu.ram.read_byte(second_byte) as u16;
                let high_byte = (cpu.ram.read_byte(second_byte + 1) as u16) << 8;
                (AddressingMode::Absolute((high_byte + lo_byte) as usize), 0)
            }
            // Absolute X-Indexed $nnnn,X
            // This form of addressing is used in conjunction with the X index register. The effective address is formed by adding the contents of X to the address contained in the second and third bytes of the instruction. This mode allows the index register to contain the index or count value and the instruction to contain the base address. This type of indexing allows any location referencing and the index to modify multiple fields resulting in reduced coding and execution time.
            AddressingMode::AbsoluteXIndex(second_byte) => {
                let lo_byte = cpu.ram.read_byte(second_byte) as u16;
                let high_byte = (cpu.ram.read_byte(second_byte + 1) as u16) << 8;
                let address = (high_byte + lo_byte) + cpu.registers.x_register as u16;
                let additional_cycles: u8 = if (address & 0xFF00) != high_byte {
                    1
                } else {
                    0
                };
                (
                    AddressingMode::AbsoluteXIndex((address) as usize),
                    additional_cycles,
                )
            }
            // Absolute Y-Indexed $nnnn,Y
            // This form of addressing is used in conjunction with the Y index register. The effective address is formed by adding the contents of Y to the address contained in the second and third bytes of the instruction. This mode allows the index register to contain the index or count value and the instruction to contain the base address. This type of indexing allows any location referencing and the index to modify multiple fields resulting in reduced coding and execution time.
            AddressingMode::AbsoluteYIndex(second_byte) => {
                let lo_byte = cpu.ram.read_byte(second_byte) as u16;
                let high_byte = (cpu.ram.read_byte(second_byte + 1) as u16) << 8;
                let address = (high_byte + lo_byte) + cpu.registers.y_register as u16;
                let additional_cycles: u8 = if (address & 0xFF00) != high_byte {
                    1
                } else {
                    0
                };
                (
                    AddressingMode::AbsoluteYIndex((address) as usize),
                    additional_cycles,
                )
            }
            // Absolute Indirect ($nnnn)
            // The second byte of the instruction contains the low order eight bits of a memory location. The high order eight bits of that memory location is contained in the third byte of the instruction. The contents of the fully specified memory location is the low order byte of the effective address. The next memory location contains the high order byte of the effective address which is loaded into the sixteen bits of the program counter.
            AddressingMode::AbsoluteIndirect(second_byte) => {
                let lo_byte = cpu.ram.read_byte(second_byte) as u16;
                let high_byte = (cpu.ram.read_byte(second_byte + 1) as u16) << 8;
                let indirect_address = (high_byte + lo_byte) as usize;
                let indirect_lo_byte = cpu.ram.read_byte(indirect_address) as u16;
                // Hardware bug in 6502 - cannot cross page boundary so read for high byte wraps to beginning of the page
                let indirect_high_byte = if indirect_address & 0x00FF == 0x00FF {
                    (cpu.ram.read_byte(indirect_address & 0xFF00) as u16) << 8
                } else {
                    (cpu.ram.read_byte(indirect_address + 1) as u16) << 8
                };
                (
                    AddressingMode::AbsoluteIndirect(
                        (indirect_high_byte + indirect_lo_byte) as usize,
                    ),
                    0,
                )
            }
            // Zero Page $nn
            // The zero page instructions allow for shorter code and execution times by only fetching the second byte of the instruction and assuming a zero high address byte. Careful use of the zero page can result in significant increase in code efficiency.
            AddressingMode::ZeroPage(second_byte) => (
                AddressingMode::ZeroPage(cpu.ram.read_byte(second_byte) as usize),
                0,
            ),
            // Zero Page X-Indexed $nn,X
            // This form of addressing is used in conjunction with the X index register. The effective address is calculated by adding the second byte to the contents of the index register. Since this is a form of "Zero Page" addressing, the content of the second byte references a location in page zero. Additionally, due to the “Zero Page" addressing nature of this mode, no carry is added to the high order 8 bits of memory and crossing of page boundaries does not occur.
            AddressingMode::ZeroPageXIndex(second_byte) => (
                AddressingMode::ZeroPageXIndex(
                    ((cpu.ram.read_byte(second_byte) as u16 + cpu.registers.x_register as u16)
                        % 0x0100) as usize,
                ),
                0,
            ),
            // Zero Page Y-Indexed $nn,Y
            // This form of addressing is used in conjunction with the Y index register. The effective address is calculated by adding the second byte to the contents of the index register. Since this is a form of "Zero Page" addressing, the content of the second byte references a location in page zero. Additionally, due to the “Zero Page" addressing nature of this mode, no carry is added to the high order 8 bits of memory and crossing of page boundaries does not occur.
            AddressingMode::ZeroPageYIndex(second_byte) => (
                AddressingMode::ZeroPageYIndex(
                    ((cpu.ram.read_byte(second_byte) as u16 + cpu.registers.y_register as u16)
                        % 0x0100) as usize,
                ),
                0,
            ),
            // Zero Page X-Indexed Indirect ($nn,X)
            // In indexed indirect addressing, the second byte of the instruction is added to the contents of the X index register, discarding the carry. The result of this addition points to a memory location on page zero whose contents is the low order eight bits of the effective address. The next memory location in page zero contains the high order eight bits of the effective address. Both memory locations specifying the high and low order bytes of the effective address must be in page zero.
            AddressingMode::ZeroPageXIndexIndirect(second_byte) => {
                let zero_indirect_address = ((cpu.ram.read_byte(second_byte) as u16
                    + cpu.registers.x_register as u16)
                    % 0x0100) as usize;
                let lo_byte = cpu.ram.read_byte(zero_indirect_address) as u16;
                let high_byte =
                    (cpu.ram.read_byte((zero_indirect_address + 1) % 0x0100) as u16) << 8;
                (
                    AddressingMode::ZeroPageXIndexIndirect((high_byte + lo_byte) as usize),
                    0,
                )
            }
            // Zero Page Y-Indexed Indirect ($nn),Y
            // In indirect indexed addressing, the second byte of the instruction points to a memory location in page zero. The contents of this memory location is added to the contents of the Y index register, the result being the low order eight bits of the effective address. The carry from this addition is added to the contents of the next page zero memory location, the result being the high order eight bits of the effective address.
            AddressingMode::ZeroPageYIndexIndirect(second_byte) => {
                let zero_address = cpu.ram.read_byte(second_byte) as usize;
                let lo_byte = cpu.ram.read_byte(zero_address) as u16;
                let high_byte = (cpu.ram.read_byte((zero_address + 1) % 0x0100) as u16) << 8;
                let zero_indirect_address = (high_byte + lo_byte) + cpu.registers.y_register as u16;
                let additional_cycles: u8 = if (zero_indirect_address & 0xFF00) != high_byte {
                    1
                } else {
                    0
                };
                (
                    AddressingMode::ZeroPageYIndexIndirect(zero_indirect_address as usize),
                    additional_cycles,
                )
            }
            // Relative $nnnn
            // Relative addressing is used only with branch instructions and establishes a destination for the conditional branch.
            // The second byte of-the instruction becomes the operand which is an “Offset" added to the contents of the lower eight bits of the program counter when the counter is set at the next instruction. The range of the offset is —128 to +127 bytes from the next instruction.
            AddressingMode::Relative(second_byte) => {
                let mut offset = cpu.ram.read_byte(second_byte) as usize;

                if offset & 0x80 == 0b1000_0000 {
                    offset += 0xFF00;
                }

                (AddressingMode::Relative(offset), 0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_state() -> Cpu {
        let mut cpu = Cpu::new();

        cpu.registers.accumulator = 0x10;
        cpu.registers.x_register = 0x03;
        cpu.registers.y_register = 0x15;

        let mut value: u8 = 0x00;
        let mut address: usize = 0x0000;
        // Creating junk data to be read
        loop {
            cpu.ram.write_byte(address, value);
            value = value.wrapping_add(3);
            address += 1;
            if address > 0xFFFF {
                break;
            }
        }

        cpu
    }

    #[test]
    fn implied_mode() {
        let cpu = setup_state();
        let ad_mode = AddressingMode::Implied;

        let (ad_mode, _additional_cycles) = ad_mode.process(&cpu);

        assert_eq!(AddressingMode::Implied, ad_mode);
    }

    #[test]
    fn accumulator_mode() {
        let cpu = setup_state();
        let ad_mode = AddressingMode::Accumulator;

        let (ad_mode, _additional_cycles) = ad_mode.process(&cpu);

        assert_eq!(AddressingMode::Accumulator, ad_mode);
    }

    #[test]
    fn immediate_mode() {
        let cpu = setup_state();
        let ad_mode = AddressingMode::Immediate(cpu.registers.program_counter + 1);

        let (ad_mode, _additional_cycles) = ad_mode.process(&cpu);

        assert_eq!(AddressingMode::Immediate(0xFFFD), ad_mode);
    }

    #[test]
    fn absolute_mode() {
        let cpu = setup_state();
        let ad_mode = AddressingMode::Absolute(cpu.registers.program_counter + 1);

        let (ad_mode, _additional_cycles) = ad_mode.process(&cpu);

        assert_eq!(AddressingMode::Absolute(0xFAF7), ad_mode);
    }

    #[test]
    fn absolute_x_index_mode() {
        let cpu = setup_state();
        let ad_mode = AddressingMode::AbsoluteXIndex(cpu.registers.program_counter + 1);

        let (ad_mode, _additional_cycles) = ad_mode.process(&cpu);

        assert_eq!(AddressingMode::AbsoluteXIndex(0xFAFA), ad_mode);
    }

    #[test]
    fn absolute_y_index_mode() {
        let cpu = setup_state();
        let ad_mode = AddressingMode::AbsoluteYIndex(cpu.registers.program_counter + 1);

        let (ad_mode, _additional_cycles) = ad_mode.process(&cpu);

        assert_eq!(AddressingMode::AbsoluteYIndex(0xFB0C), ad_mode);
    }

    #[test]
    fn absolute_indirect_mode() {
        let cpu = setup_state();
        let ad_mode = AddressingMode::AbsoluteIndirect(cpu.registers.program_counter + 1);

        let (ad_mode, _additional_cycles) = ad_mode.process(&cpu);

        assert_eq!(AddressingMode::AbsoluteIndirect(0xE8E5), ad_mode);
    }

    #[test]
    fn zero_page_mode() {
        let cpu = setup_state();
        let ad_mode = AddressingMode::ZeroPage(cpu.registers.program_counter + 1);

        let (ad_mode, _additional_cycles) = ad_mode.process(&cpu);

        assert_eq!(AddressingMode::ZeroPage(0x00F7), ad_mode);
    }

    #[test]
    fn zero_page_x_index_mode() {
        let cpu = setup_state();
        let ad_mode = AddressingMode::ZeroPageXIndex(cpu.registers.program_counter + 1);

        let (ad_mode, _additional_cycles) = ad_mode.process(&cpu);

        assert_eq!(AddressingMode::ZeroPageXIndex(0x00FA), ad_mode);
    }

    #[test]
    fn zero_page_y_index_mode() {
        let cpu = setup_state();
        let ad_mode = AddressingMode::ZeroPageYIndex(cpu.registers.program_counter + 1);

        let (ad_mode, _additional_cycles) = ad_mode.process(&cpu);

        assert_eq!(AddressingMode::ZeroPageYIndex(0x000C), ad_mode);
    }

    #[test]
    fn zero_page_x_index_indirect_mode() {
        let cpu = setup_state();
        let ad_mode = AddressingMode::ZeroPageXIndexIndirect(cpu.registers.program_counter + 1);

        let (ad_mode, _additional_cycles) = ad_mode.process(&cpu);

        assert_eq!(AddressingMode::ZeroPageXIndexIndirect(0xF1EE), ad_mode);
    }

    #[test]
    fn zero_page_y_index_indirect_mode() {
        let cpu = setup_state();
        let ad_mode = AddressingMode::ZeroPageYIndexIndirect(cpu.registers.program_counter + 1);

        let (ad_mode, _additional_cycles) = ad_mode.process(&cpu);

        assert_eq!(AddressingMode::ZeroPageYIndexIndirect(0xE8FA), ad_mode);
    }

    #[test]
    fn relative_mode() {
        let cpu = setup_state();
        let ad_mode = AddressingMode::Relative(cpu.registers.program_counter + 1);

        let (ad_mode, _additional_cycles) = ad_mode.process(&cpu);

        assert_eq!(AddressingMode::Relative(0xFFF7), ad_mode);
    }
}
