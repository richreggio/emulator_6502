use super::*;

// STX - Store Index Register X In Memory
// Operation: X → M
// Transfers value of X register to addressed memory location.
// No flags or registers in the microprocessor are affected by the store operation.
// -----------------------------------------------------------------------------------------------
// | Addressing Mode	            | Assembly Language Form | Opcode	| No. Bytes	| No. Cycles |
// |--------------------------------|------------------------------------------------------------|
// | Absolute                       | STX $nnnn              | $8E      | 3         | 4          |
// | Zero Page                      | STX $nn                | $86      | 2         | 3          |
// | Zero Page Y-Indexed            | STX $nn,Y              | $96      | 2         | 4          |
// | p: =1 if page is crossed       |                        |          |           |            |
// |----------------------------------------------------------------------------------------------

pub fn stx(cpu: &mut Cpu, operation: &mut Operation) {
    let address = match operation.addressing_mode {
        AdMode::Absolute(address) => address,
        AdMode::ZeroPage(address) => address,
        AdMode::ZeroPageYIndex(address) => address,
        _ => panic!("Invalid STX operation"),
    };
    cpu.ram.write_byte(address, cpu.registers.x_register);
}
