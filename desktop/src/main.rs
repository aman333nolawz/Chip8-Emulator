use chip8_core::*;
use macroquad::prelude::*;
use std::{collections::HashSet, env, fs::File, io::Read};

const SCALE: u32 = 30;
const WINDOW_WIDTH: u32 = SCREEN_WIDTH as u32 * SCALE;
const WINDOW_HEIGHT: u32 = SCREEN_HEIGHT as u32 * SCALE;

const TICKS_PER_FRAME: usize = 20;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chip-8 Emulator".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }

    let mut chip8 = Emu::new();
    let mut rom = File::open(&args[1]).expect("Unable to open the rom");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    chip8.load(&buffer);

    loop {
        clear_background(Color::from_hex(0x1E1E2E));
        if let Some(k) = key_to_btn(get_keys_down()) {
            chip8.keypress(k, true);
        }
        if let Some(k) = key_to_btn(get_keys_released()) {
            chip8.keypress(k, false);
        }
        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }
        chip8.tick_timers();
        draw_screen(&chip8);
        next_frame().await;
    }
}

fn draw_screen(emu: &Emu) {
    let screen_buf = emu.get_display();
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            draw_rectangle(
                x as f32 * SCALE as f32,
                y as f32 * SCALE as f32,
                SCALE as f32,
                SCALE as f32,
                Color::from_hex(0x89b4fa),
            );
        }
    }
}

fn key_to_btn(keys: HashSet<KeyCode>) -> Option<usize> {
    if keys.contains(&KeyCode::Key1) {return  Some(0x1);}
    else if keys.contains(&KeyCode::Key2) { Some(0x2)}
    else if keys.contains(&KeyCode::Key3) {Some(0x3)}
    else if keys.contains(&KeyCode::Key4) {Some(0xC)}
    else if keys.contains(&KeyCode::Q)  {Some(0x4)}
    else if keys.contains(&KeyCode::W)  {Some(0x5)}
    else if keys.contains(&KeyCode::E)  {Some(0x6)}
    else if keys.contains(&KeyCode::R)  {Some(0xD)}
    else if keys.contains(&KeyCode::A)  {Some(0x7)}
    else if keys.contains(&KeyCode::S)  {Some(0x8)}
    else if keys.contains(&KeyCode::D)  {Some(0x9)}
    else if keys.contains(&KeyCode::F)  {Some(0xE)}
    else if keys.contains(&KeyCode::Z)  {Some(0xA)}
    else if keys.contains(&KeyCode::X)  {Some(0x0)}
    else if keys.contains(&KeyCode::C)  {Some(0xB)}
    else if keys.contains(&KeyCode::V)  {Some(0xF)}
    else {None}    
}
