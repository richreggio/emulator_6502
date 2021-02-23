use super::*;

// ROL - Rotate Left
// Operation: C ← /M7...M0/ ← C
// The rotate left instruction shifts either the accumulator or addressed memory left 1 bit, with the input carry being stored in bit 0 and with the input bit 7 being stored in the carry flags.
// The ROL instruction either shifts the accumulator left 1 bit and stores the carry in accumulator bit 0 or does not affect the internal reg­isters at all. The ROL instruction sets carry equal to the input bit 7, sets N equal to the input bit 6 , sets the Z flag if the result of the ro­ tate is 0, otherwise it resets Z and does not affect the overflow flag at all.
// -----------------------------------------------------------------------------------------------
// | Addressing Mode	              | Assembly Language Form | Opcode   | No. Bytes	| No. Cycles |
// |--------------------------------|------------------------------------------------------------|
// | Accumulator                    | ROL A   	             | $2A      | 1         | 2          |
// | Absolute                       | ROL $nnnn              | $2E      | 3         | 6          |
// | Absolute X-Indexed             | ROL $nnnn,X            | $3E      | 3         | 7          |
// | Zero Page                      | ROL $nn                | $26      | 2         | 5          |
// | Zero Page X-Indexed            | ROL $nn,X              | $36      | 2         | 6          |
// |----------------------------------------------------------------------------------------------

pub fn rol(_memory: &mut Memory, _registers: &mut Registers, _operation: Operation) {}
