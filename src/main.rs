use emulator_6502::cpu::operation::Operation;
use emulator_6502::cpu::registers::Registers;
use emulator_6502::memory::Memory;
// extern crate sdl2;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
// use std::time::Duration;

fn main() {
    // Create the 6502 and memory
    let mut memory = Memory::new_initalized();
    let mut registers = Registers::new_initalized();

    // // Setting up window
    // let sdl_context = sdl2::init().unwrap();
    // let video_subsystem = sdl_context.video().unwrap();

    // let window = video_subsystem
    //     .window("richreggio's NES emulator", 800, 600)
    //     .position_centered()
    //     .build()
    //     .unwrap();

    // let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();
    // let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut i = 0;
    // 'running: loop {
    //     i = (i + 1) % 255;
    //     canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    //     canvas.clear();
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit { .. }
    //             | Event::KeyDown {
    //                 keycode: Some(Keycode::Escape),
    //                 ..
    //             } => break 'running,
    //             _ => {}
    //         }
    //     }
    //     // The rest of the game loop goes here...

    //     canvas.present();
    //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    // }

    loop {
        // Get next operation
        let operation = Operation::next(&mut registers, &memory);
        // Perform the next operation
        execute(&mut memory, &mut registers, operation);
    }
}

fn execute(memory: &mut Memory, registers: &mut Registers, operation: Operation) {
    (operation.instruction_function)(memory, registers, operation);
}
