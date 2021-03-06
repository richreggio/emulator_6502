use super::*;

// STY - Store Index Register Y In Memory
// Operation: Y → M
// Transfer the value of the Y register to the addressed memory location.
// STY does not affect any flags or registers in the microprocessor.
// -----------------------------------------------------------------------------------------------
// | Addressing Mode	            | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |--------------------------------|------------------------------------------------------------|
// | Absolute                       | STY $nnnn              | $8C      | 3         | 4          |
// | Zero Page                      | STY $nn                | $84      | 2         | 3          |
// | Zero Page X-Indexed            | STY $nn,X              | $94      | 2         | 4          |
// | p: =1 if page is crossed       |                        |          |           |            |
// |----------------------------------------------------------------------------------------------

pub fn sty(cpu: &mut Cpu, operation: &mut Operation) {
    let address = match operation.addressing_mode {
        AdMode::Absolute(address) => address,
        AdMode::ZeroPage(address) => address,
        AdMode::ZeroPageYIndex(address) => address,
        _ => panic!("Invalid STY operation"),
    };
    cpu.ram.write_byte(address, cpu.registers.y_register);
}
