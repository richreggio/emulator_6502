use super::*;

// SLO - Arithmetic Shift Left then "OR" Memory with Accumulator
// Operation: M * 2 → M, A ∨ M → A
// The undocumented SLO instruction shifts the address memory location 1 bit to the left, with the bit 0 always being set to 0 and the bit 7 output always being contained in the carry flag. It then performs a bit-by-bit "OR" operation on the result and the accumulator and stores the result in the accumulator.
// The negative flag is set if the accumulator result contains bit 7 on, otherwise the negative flag is reset. It sets Z flag if the result is equal to 0, otherwise resets Z and stores the input bit 7 in the carry flag.
// -----------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode	  | No. Bytes	| No. Cycles |
// |--------------------------------|------------------------------------------------------------|
// | Absolute                       | SLO $nnnn              | $0F*     | 3	        | 6          |
// | Absolute X-Indexed             | SLO $nnnn,X            | $1F*     | 3         | 7          |
// | Absolute Y-Indexed	            | SLO $nnnn,Y            | $1B*     | 3	        | 7          |
// | Zero Page                      | SLO $nn                | $07*     | 2	        | 5          |
// | Zero Page X-Indexed            | SLO $nn,X              | $17*     | 2	        | 6          |
// | Zero Page X-Indexed Indirect   | SLO ($nn,X)            | $03*     | 2	        | 8          |
// | Zero Page Y-Indexed Indirect   | SLO ($nn),Y            | $13*     | 2	        | 8          |
// |----------------------------------------------------------------------------------------------

pub fn slo(_cpu: &mut Cpu, _operation: &mut Operation) {}
