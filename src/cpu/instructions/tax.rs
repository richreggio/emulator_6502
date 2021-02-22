use super::*;

// TAX - Transfer Accumulator To Index X
// Operation: A → X
// This instruction takes the value from accumulator A and trans­ fers or loads it into the index register X without disturbing the content of the accumulator A.
// TAX only affects the index register X, does not affect the carry or overflow flags. The N flag is set if the resultant value in the index register X has bit 7 on, otherwise N is reset. The Z bit is set if the content of the register X is 0 as aresult of theopera­ tion, otherwise it is reset.
// Addressing Mode	Assembly Language Form	Opcode	No. Bytes	No. Cycles
// Implied	TAX 	$AA	1	2

pub fn tax(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
