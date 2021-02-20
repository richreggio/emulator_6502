// use emulator_6502::cpu::addressing_mode;
use emulator_6502::cpu::registers::Registers;
use emulator_6502::memory::Memory;

fn print_stack_value(memory: &Memory, registers: &Registers) {
    println!(
        "Stack pointer: {}, Stack Value: {}",
        registers.stack_pointer,
        memory.read_byte(0x0100 & registers.stack_pointer as usize)
    );
}

fn main() {
    let mut registers = Registers::new_initalized(0xFFFF);
    let mut memory = Memory::new_initalized();
    println!("The current state of the CPU is : {:?}", registers);

    registers.stack_push(&mut memory, 0xFF);
    print_stack_value(&memory, &registers);
}
