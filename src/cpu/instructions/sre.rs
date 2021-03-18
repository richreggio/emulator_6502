use super::*;

// SRE - Logical Shift Right then "Exclusive OR" Memory with Accumulator
// Operation: M / 2 → M, A ⊻ M → A
// The undocumented SRE instruction shifts the specified memory location 1 bit to the right, with the higher bit of the result always being set to 0, and the low bit which is shifted out of the field being stored in the carry flag. It then performs a bit-by-bit "EXCLUSIVE OR" of the result and the value of the accumulator and stores the result in the accumulator.
// This instruction affects the accumulator. It does not affect the overflow flag. The negative flag is set if the accumulator result contains bit 7 on, otherwise the negative flag is reset. The Z flag is set if the result is 0 and reset otherwise. The carry is set equal to input bit 0.
// -----------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode	  | No. Bytes	| No. Cycles |
// |--------------------------------|------------------------------------------------------------|
// | Absolute                       | SRE $nnnn              | $4F*     | 3	        | 6          |
// | Absolute X-Indexed             | SRE $nnnn,X            | $5F*     | 3         | 7          |
// | Absolute Y-Indexed	            | SRE $nnnn,Y            | $5B*     | 3	        | 7          |
// | Zero Page                      | SRE $nn                | $47*     | 2	        | 5          |
// | Zero Page X-Indexed            | SRE $nn,X              | $57*     | 2	        | 6          |
// | Zero Page X-Indexed Indirect   | SRE ($nn,X)            | $43*     | 2	        | 8          |
// | Zero Page Y-Indexed Indirect   | SRE ($nn),Y            | $53*     | 2	        | 8          |
// |----------------------------------------------------------------------------------------------

pub fn sre(_cpu: &mut CPU, _operation: &mut Operation) {}
