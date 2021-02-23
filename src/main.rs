// use emulator_6502::cpu::addressing_mode;
use emulator_6502::cpu::operation::Operation;
use emulator_6502::cpu::registers::Registers;
use emulator_6502::memory::Memory;

fn main() {
    let mut memory = Memory::new_initalized();
    let mut registers = Registers::new_initalized(0xFFFF);
    println!("The current state of the CPU is : {:?}", registers);

    loop {
        let operation = Operation::next(&mut registers, &memory);
        execute(&mut memory, &mut registers, operation);
    }
}

fn execute(memory: &mut Memory, registers: &mut Registers, operation: Operation) {
    (operation.instruction_function)(memory, registers, operation);
}
