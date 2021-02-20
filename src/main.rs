// use emulator_6502::cpu::addressing_mode;
use emulator_6502::cpu::registers::Registers;
use emulator_6502::memory::Memory;

fn main() {
    let registers = Registers::new_initalized(0xFFFF);
    let _memory = Memory::new_initalized();
    println!("The current state of the CPU is : {:?}", registers);
    // Execute code
}
