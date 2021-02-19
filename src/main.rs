use emulator_6502::cpu::registers::Registers;
use emulator_6502::memory::Memory;

fn main() {
    let mut registers = Registers::new(0xFFFF);
    let mut memory = Memory::new();
    println!("The current state of the CPU is : {:?}", registers);

    registers.initalize(0x0100);
    memory.initalize();
    println!("The current state of the CPU is : {:?}", registers);
}
