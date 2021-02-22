// use emulator_6502::cpu::addressing_mode;
use emulator_6502::cpu::operation::Operation;
use emulator_6502::cpu::registers::Registers;
use emulator_6502::memory::Memory;

fn main() {
    let mut registers = Registers::new_initalized(0xFFFF);
    let mut memory = Memory::new_initalized();
    println!("The current state of the CPU is : {:?}", registers);

    loop {
        execute(&mut registers, &mut memory);
    }
}

fn execute(registers: &mut Registers, memory: &mut Memory) {
    Operation::next(
        memory.read_byte(registers.program_counter),
        registers.program_counter + 1,
    );
}
