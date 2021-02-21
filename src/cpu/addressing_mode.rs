pub enum AddressingMode {
  Implied,
  Accumulator,
  Immediate(u8),
  Absolute(u8, u8),
  AbsoluteXIndex(u8, u8),
  AbsoluteYIndex(u8, u8),
  AbsoluteIndirect(u8, u8),
  ZeroPage(u8),
  ZeroPageXIndex(u8),
  ZeroPageYIndex(u8),
  ZeroPageXIndexIndirect(u8),
  ZeroPageYIndexIndirect(u8),
  Relative(u8),
}

impl AddressingMode {}

// Implied
// In the implied addressing mode, the address containing the operand is implicitly stated in the operation code of the instruction.
// Bytes: 1

// Accumulator
// This form of addressing is represented with a one byte instruction, implying an operation on the accumulator.
// Bytes: 1

// Immediate #$nn
// In immediate addressing, the operand is contained in the second byte of the instruction, with no further memory addressing required.
// Bytes: 2

// Absolute $nnnn
// In absolute addressing, the second byte of the instruction specifies the eight low order bits of the effective address while the third byte specifies the eight high order bits. Thus, the absolute addressing mode allows access to the entire 65 K bytes of addressable memory.
// Bytes: 3

// Absolute X-Indexed $nnnn,X
// This form of addressing is used in conjunction with the X index register. The effective address is formed by adding the contents of X to the address contained in the second and third bytes of the instruction. This mode allows the index register to contain the index or count value and the instruction to contain the base address. This type of indexing allows any location referencing and the index to modify multiple fields resulting in reduced coding and execution time.
// Note on the MOS 6502:
// The value at the specified address, ignoring the the addressing mode's X offset, is read (and discarded) before the final address is read. This may cause side effects in I/O registers.
// Bytes: 3

// Absolute Y-Indexed $nnnn,Y
// This form of addressing is used in conjunction with the Y index register. The effective address is formed by adding the contents of Y to the address contained in the second and third bytes of the instruction. This mode allows the index register to contain the index or count value and the instruction to contain the base address. This type of indexing allows any location referencing and the index to modify multiple fields resulting in reduced coding and execution time.
// Note on the MOS 6502:
// The value at the specified address, ignoring the the addressing mode's Y offset, is read (and discarded) before the final address is read. This may cause side effects in I/O registers.
// Bytes: 3

// Absolute Indirect ($nnnn)
// The second byte of the instruction contains the low order eight bits of a memory location. The high order eight bits of that memory location is contained in the third byte of the instruction. The contents of the fully specified memory location is the low order byte of the effective address. The next memory location contains the high order byte of the effective address which is loaded into the sixteen bits of the program counter.
// Note on the MOS 6502:
// The indirect jump instruction does not increment the page address when the indirect pointer crosses a page boundary. JMP ($xxFF) will fetch the address from $xxFF and $xx00.
// Bytes: 3

// Zero Page $nn
// The zero page instructions allow for shorter code and execution times by only fetching the second byte of the instruction and assuming a zero high address byte. Careful use of the zero page can result in significant increase in code efficiency.
// Bytes: 2

// Zero Page X-Indexed $nn,X
// This form of addressing is used in conjunction with the X index register. The effective address is calculated by adding the second byte to the contents of the index register. Since this is a form of "Zero Page" addressing, the content of the second byte references a location in page zero. Additionally, due to the “Zero Page" addressing nature of this mode, no carry is added to the high order 8 bits of memory and crossing of page boundaries does not occur.
// Bytes: 2

// Zero Page Y-Indexed $nn,Y
// This form of addressing is used in conjunction with the Y index register. The effective address is calculated by adding the second byte to the contents of the index register. Since this is a form of "Zero Page" addressing, the content of the second byte references a location in page zero. Additionally, due to the “Zero Page" addressing nature of this mode, no carry is added to the high order 8 bits of memory and crossing of page boundaries does not occur.
// Bytes: 2

// Zero Page X-Indexed Indirect ($nn,X)
// In indexed indirect addressing, the second byte of the instruction is added to the contents of the X index register, discarding the carry. The result of this addition points to a memory location on page zero whose contents is the low order eight bits of the effective address. The next memory location in page zero contains the high order eight bits of the effective address. Both memory locations specifying the high and low order bytes of the effective address must be in page zero.
// Bytes: 2

// Zero Page Y-Indexed Indirect ($nn),Y
// In indirect indexed addressing, the second byte of the instruction points to a memory location in page zero. The contents of this memory location is added to the contents of the Y index register, the result being the low order eight bits of the effective address. The carry from this addition is added to the contents of the next page zero memory location, the result being the high order eight bits of the effective address.
// Bytes: 2

// Relative $nnnn
// Relative addressing is used only with branch instructions and establishes a destination for the conditional branch.
// The second byte of-the instruction becomes the operand which is an “Offset" added to the contents of the lower eight bits of the program counter when the counter is set at the next instruction. The range of the offset is —128 to +127 bytes from the next instruction.
// Bytes: 2
