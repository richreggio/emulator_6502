use super::*;

// CLV - Clear Overflow Flag
// Operation: 0 → V
// This instruction clears the overflow flag to a 0. This com­ mand is used in conjunction with the set overflow pin which can change the state of the overflow flag with an external signal.
// CLV affects no registers in the microprocessor and no flags other than the overflow flag which is set to a 0.
// ---------------------------------------------------------------------------------------------
// | Addressing Mode                | Assembly Language Form | Opcode | No. Bytes | No. Cycles |
// |--------------------------------|----------------------------------------------------------|
// | Implied                        | CLV                    | $B8    |	1         | 2          |
// |--------------------------------------------------------------------------------------------

pub fn clv(cpu: &mut Cpu, _operation: &mut Operation) {
    cpu.registers.set_overflow_flag(false);
}
