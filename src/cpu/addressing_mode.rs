use crate::cpu::registers::Registers;
use crate::memory::Memory;

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
    pub fn process(&self, registers: &Registers, memory: &Memory) -> AddressingMode {
        match *self {
            // Implied
            // In the implied addressing mode, the address containing the operand is implicitly stated in the operation code of the instruction.
            AddressingMode::Implied => AddressingMode::Implied,
            // Accumulator
            // This form of addressing is represented with a one byte instruction, implying an operation on the accumulator.
            AddressingMode::Accumulator => AddressingMode::Accumulator,
            // Immediate #$nn
            // In immediate addressing, the operand is contained in the second byte of the instruction, with no further memory addressing required.
            AddressingMode::Immediate(second_byte) => AddressingMode::Immediate(second_byte),
            // Absolute $nnnn
            // In absolute addressing, the second byte of the instruction specifies the eight low order bits of the effective address while the third byte specifies the eight high order bits. Thus, the absolute addressing mode allows access to the entire 65 K bytes of addressable memory.
            AddressingMode::Absolute(second_byte) => {
                let lo_byte = memory.read_byte(second_byte);
                let high_byte = memory.read_byte(second_byte + 1) << 8;
                AddressingMode::Absolute((high_byte + lo_byte) as usize)
            }
            // Absolute X-Indexed $nnnn,X
            // This form of addressing is used in conjunction with the X index register. The effective address is formed by adding the contents of X to the address contained in the second and third bytes of the instruction. This mode allows the index register to contain the index or count value and the instruction to contain the base address. This type of indexing allows any location referencing and the index to modify multiple fields resulting in reduced coding and execution time.
            AddressingMode::AbsoluteXIndex(second_byte) => {
                let lo_byte = memory.read_byte(second_byte);
                let high_byte = memory.read_byte(second_byte + 1) << 8;
                AddressingMode::AbsoluteXIndex(
                    ((high_byte + lo_byte) + registers.x_register) as usize,
                )
            }
            // Absolute Y-Indexed $nnnn,Y
            // This form of addressing is used in conjunction with the Y index register. The effective address is formed by adding the contents of Y to the address contained in the second and third bytes of the instruction. This mode allows the index register to contain the index or count value and the instruction to contain the base address. This type of indexing allows any location referencing and the index to modify multiple fields resulting in reduced coding and execution time.
            AddressingMode::AbsoluteYIndex(second_byte) => {
                let lo_byte = memory.read_byte(second_byte);
                let high_byte = memory.read_byte(second_byte + 1) << 8;
                AddressingMode::AbsoluteXIndex(
                    ((high_byte + lo_byte) + registers.y_register) as usize,
                )
            }
            // Absolute Indirect ($nnnn)
            // The second byte of the instruction contains the low order eight bits of a memory location. The high order eight bits of that memory location is contained in the third byte of the instruction. The contents of the fully specified memory location is the low order byte of the effective address. The next memory location contains the high order byte of the effective address which is loaded into the sixteen bits of the program counter.
            AddressingMode::AbsoluteIndirect(second_byte) => {
                let lo_byte = memory.read_byte(second_byte);
                let high_byte = memory.read_byte(second_byte + 1) << 8;
                let indirect_address = (high_byte + lo_byte) as usize;

                let indirect_lo_byte = memory.read_byte(indirect_address);
                let indirect_high_byte = memory.read_byte(indirect_address + 1) << 8;
                AddressingMode::AbsoluteIndirect((high_byte + lo_byte) as usize)
            }
            // Zero Page $nn
            // The zero page instructions allow for shorter code and execution times by only fetching the second byte of the instruction and assuming a zero high address byte. Careful use of the zero page can result in significant increase in code efficiency.
            AddressingMode::ZeroPage(second_byte) => {
                AddressingMode::ZeroPage(memory.read_byte(second_byte) as usize)
            }
            // Zero Page X-Indexed $nn,X
            // This form of addressing is used in conjunction with the X index register. The effective address is calculated by adding the second byte to the contents of the index register. Since this is a form of "Zero Page" addressing, the content of the second byte references a location in page zero. Additionally, due to the “Zero Page" addressing nature of this mode, no carry is added to the high order 8 bits of memory and crossing of page boundaries does not occur.
            AddressingMode::ZeroPageXIndex(second_byte) => AddressingMode::ZeroPageXIndex(
                ((memory.read_byte(second_byte) + registers.x_register) % 0x0100) as usize,
            ),
            // Zero Page Y-Indexed $nn,Y
            // This form of addressing is used in conjunction with the Y index register. The effective address is calculated by adding the second byte to the contents of the index register. Since this is a form of "Zero Page" addressing, the content of the second byte references a location in page zero. Additionally, due to the “Zero Page" addressing nature of this mode, no carry is added to the high order 8 bits of memory and crossing of page boundaries does not occur.
            AddressingMode::ZeroPageYIndex(second_byte) => AddressingMode::ZeroPageYIndex(
                ((memory.read_byte(second_byte) + registers.y_register) % 0x0100) as usize,
            ),
            // Zero Page X-Indexed Indirect ($nn,X)
            // In indexed indirect addressing, the second byte of the instruction is added to the contents of the X index register, discarding the carry. The result of this addition points to a memory location on page zero whose contents is the low order eight bits of the effective address. The next memory location in page zero contains the high order eight bits of the effective address. Both memory locations specifying the high and low order bytes of the effective address must be in page zero.
            AddressingMode::ZeroPageXIndexIndirect(second_byte) => {
                let zero_indirect_address =
                    ((memory.read_byte(second_byte) + registers.x_register) % 0x0100) as usize;
                let lo_byte = memory.read_byte(zero_indirect_address);
                let high_byte = memory.read_byte(zero_indirect_address + 1) << 8;
                AddressingMode::ZeroPageXIndexIndirect((high_byte + lo_byte) as usize)
            }
            // Zero Page Y-Indexed Indirect ($nn),Y
            // In indirect indexed addressing, the second byte of the instruction points to a memory location in page zero. The contents of this memory location is added to the contents of the Y index register, the result being the low order eight bits of the effective address. The carry from this addition is added to the contents of the next page zero memory location, the result being the high order eight bits of the effective address.
            AddressingMode::ZeroPageYIndexIndirect(second_byte) => {
                let zero_indirect_address =
                    ((memory.read_byte(second_byte) + registers.y_register) % 0x0100) as usize;
                let lo_byte = memory.read_byte(zero_indirect_address);
                let high_byte = memory.read_byte(zero_indirect_address + 1) << 8;
                AddressingMode::ZeroPageYIndexIndirect((high_byte + lo_byte) as usize)
            }
            // Relative $nnnn
            // Relative addressing is used only with branch instructions and establishes a destination for the conditional branch.
            // The second byte of-the instruction becomes the operand which is an “Offset" added to the contents of the lower eight bits of the program counter when the counter is set at the next instruction. The range of the offset is —128 to +127 bytes from the next instruction.
            AddressingMode::Relative(second_byte) => {
                let offset = memory.read_byte(second_byte);
                AddressingMode::Relative(offset as usize)
            }
        }
    }
}
