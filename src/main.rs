// use emulator_6502::cpu::addressing_mode;
use emulator_6502::cpu::operation::Operation;
use emulator_6502::cpu::registers::Registers;
use emulator_6502::memory::Memory;
use std::fs::read;

fn main() {
    let mut memory = Memory::new_initalized();
    let mut registers = Registers::new_initalized();

    load_roms(&mut memory);
    display_status(&registers);

    loop {
        let operation = Operation::next(&mut registers, &memory);
        execute(&mut memory, &mut registers, operation);
    }
}

fn execute(memory: &mut Memory, registers: &mut Registers, operation: Operation) {
    (operation.instruction_function)(memory, registers, operation);
}

fn display_status(registers: &Registers) {
    println!("The current state of the CPU is : {:?}", registers);
}

fn load_roms(memory: &mut Memory) {
    let basic_rom_data = read("/roms/basic.901226-01.bin").unwrap();
    let kernal_rom_data = read("/roms/kernal.901227-03.bin").unwrap();
    let character_rom_data = read("/roms/characters.901225-01.bin").unwrap();

    // BASIC ROM visible at $A000-$BFFF
    let mut target_address = 0xA000;
    for value in basic_rom_data {
        memory.write_byte(target_address, value);
        target_address += 1
    }

    // KERNAL ROM visible at $E000-$FFFF
    target_address = 0xE000;
    for value in kernal_rom_data {
        memory.write_byte(target_address, value);
        target_address += 1
    }

    // Character ROM visible at $D000-$DFFF
    target_address = 0xD000;
    for value in character_rom_data {
        memory.write_byte(target_address, value);
        target_address += 1
    }
}
