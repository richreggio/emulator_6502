use super::*;

// LAX - Load Accumulator and Index Register X From Memory
// Operation: M → A, X
// The undocumented LAX instruction loads the accumulator and the index register X from memory.
// LAX does not affect the C or V flags; sets Z if the value loaded was zero, otherwise resets it; sets N if the value loaded in bit 7 is a 1; otherwise N is reset, and affects only the X register.
// -------------------------------------------------------------------------------------------
// | Addressing Mode	          | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |------------------------------|----------------------------------------------------------|
// | Immediate                    | LAX #$nn               | $AB*	| 2	        | 2          |
// | Absolute	                  | LAX $nnnn              | $AF*	| 3	        | 4          |
// | Absolute Y-Indexed	          | LAX $nnnn,Y	           | $BF*	| 3	        | 4+p        |
// | Zero Page	                  | LAX $nn	               | $A7*	| 2	        | 3          |
// | Zero Page Y-Indexed	      | LAX $nn,Y              | $B7*	| 2	        | 4          |
// | Zero Page X-Indexed Indirect |	LAX ($nn,X)	           | $A3*	| 2	        | 6          |
// | Zero Page Y-Indexed Indirect |	LAX ($nn),Y	           | $B3*	| 2	        | 5+p        |
// | p: =1 if page is crossed.    |                        |        |           |            |
// |------------------------------------------------------------------------------------------

pub fn lax(memory: &mut Memory, registers: &mut Registers, operation: Operation) {
    let value = match operation.addressing_mode {
        AdMode::Immediate(address) => memory.read_byte(address),
        AdMode::Absolute(address) => memory.read_byte(address),
        AdMode::AbsoluteYIndex(address) => memory.read_byte(address),
        AdMode::ZeroPage(address) => memory.read_byte(address),
        AdMode::ZeroPageYIndex(address) => memory.read_byte(address),
        AdMode::ZeroPageXIndexIndirect(address) => memory.read_byte(address),
        AdMode::ZeroPageYIndexIndirect(address) => memory.read_byte(address),
        _ => 0,
    };

    registers.accumulator = value;
    registers.x_register = value;

    if value == 0 {
        registers.set_zero_flag(true)
    } else {
        registers.set_zero_flag(false)
    }

    // Checking seventh bit of value
    if (value & 0b1000_0000) == 0b1000_0000 {
        registers.set_negative_flag(true)
    } else {
        registers.set_negative_flag(false)
    }
}
