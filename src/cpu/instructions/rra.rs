use super::*;

// RRA - Rotate Right and Add Memory to Accumulator
// Operation: C → /M7...M0/ → C, A + M + C → A
// The undocumented RRA instruction shifts the addressed memory right 1 bit with bit 0 shifted into the carry and carry shifted into bit 7. It then adds the result and generated carry to the value of the accumulator and stores the result in the accumulator.
// This instruction affects the accumulator; sets the carry flag when the sum of a binary add exceeds 255 or when the sum of a decimal add exceeds 99, otherwise carry is reset. The overflow flag is set when the sign or bit 7 is changed due to the result exceeding +127 or -128, otherwise overflow is reset. The negative flag is set if the accumulator result contains bit 7 on, otherwise the negative flag is reset. The zero flag is set if the accumulator result is 0, otherwise the zero flag is reset.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Absolute                       | RRA $nnnn              | $6F*   |	3         | 6          |
// | Absolute X-Indexed             | RRA $nnnn,X            | $7F*   | 3         | 7          |
// | Absolute Y-Indexed	            | RRA $nnnn,Y            | $7B*   | 3         | 7          |
// | Zero Page                      | RRA $nn                | $67*   | 2         | 5          |
// | Zero Page X-Indexed            | RRA $nn,X              | $77*   | 2         | 6          |
// | Zero Page X-Indexed Indirect   | RRA ($nn,X)            | $63*   | 2         | 8          |
// | Zero Page Y-Indexed Indirect   | RRA ($nn),Y            | $73*   | 2         | 8          |
// |--------------------------------------------------------------------------------------------

pub fn rra(_cpu: &mut Cpu, _operation: &mut Operation) {}
