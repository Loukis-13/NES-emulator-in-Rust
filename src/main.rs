mod bus;
mod controller;
mod cpu;
mod ppu;
mod render;
mod rom;

use std::collections::HashMap;

use bus::Bus;
use cpu::CPU;
use ppu::NesPPU;
use render::Frame;
use rom::Rom;
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum};

fn main() {
    let game_name = "/home/loukis/nesgames/Ice Climber (USA, Europe).nes";

    // init sdl2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(game_name.trim_end_matches(".nes"), 256 * 3, 240 * 3)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(3.0, 3.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator.create_texture_target(PixelFormatEnum::RGB24, 256, 240).unwrap();

    // config controller
    let mut key_map = HashMap::new();
    key_map.insert(Keycode::Down, controller::Joypad::DOWN);
    key_map.insert(Keycode::Up, controller::Joypad::UP);
    key_map.insert(Keycode::Right, controller::Joypad::RIGHT);
    key_map.insert(Keycode::Left, controller::Joypad::LEFT);
    key_map.insert(Keycode::Space, controller::Joypad::SELECT);
    key_map.insert(Keycode::Return, controller::Joypad::START);
    key_map.insert(Keycode::A, controller::Joypad::A);
    key_map.insert(Keycode::S, controller::Joypad::B);

    //load the game
    let game_code = std::fs::read(game_name).unwrap();
    let rom = Rom::new(&game_code).unwrap();

    let mut frame = Frame::new();

    // the game cycle
    let bus = Bus::new(rom, move |ppu: &NesPPU, joypad: &mut controller::Joypad| {
        render::render(ppu, &mut frame);
        texture.update(None, &frame.data, 256 * 3).unwrap();

        canvas.copy(&texture, None, None).unwrap();

        canvas.present();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),

                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad.set_button_pressed_status(*key, true);
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        joypad.set_button_pressed_status(*key, false);
                    }
                }

                _ => { /* do nothing */ }
            }
        }
    });

    let mut cpu = CPU::new(bus);

    cpu.reset();
    cpu.run();
}
