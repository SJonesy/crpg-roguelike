extern crate sdl2;

mod renderer;

use std::fs;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::InitFlag;


const BUFF_SIZE: usize = 1048576;


fn main() {
    // Setup
    let sdl_context = sdl2::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    // Initialize window and define layout boundaries
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("crpg roguelite", 800, 600)
        .position_centered()
        .fullscreen_desktop()
        .resizable()
        .build()
        .unwrap();
    let screen_width = window.size().0;
    let screen_height = window.size().1;
    let mut canvas = window.into_canvas().build().unwrap();

    let mut render_state = RenderState {
        screen_width: screen_width,
        screen_height: screen_height,
        game_state: GameState::Combat,
        background: [0; BUFF_SIZE],
        enemy: [0; BUFF_SIZE]
    };

    let mut buffer: [u8; BUFF_SIZE] = [0; BUFF_SIZE];
    let _size = get_file_bytes(String::from("assets/field.png"), &mut buffer);
    render_state.background.clone_from_slice(&buffer);
    buffer = [0; BUFF_SIZE];
    let _size = get_file_bytes(String::from("assets/zombie.png"), &mut buffer);
    render_state.enemy.clone_from_slice(&buffer);

    renderer::render(&mut canvas, render_state).unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

#[derive(Debug)]
pub struct RenderState {
    pub screen_width: u32,
    pub screen_height: u32,
    pub game_state: GameState,
    pub background: [u8; BUFF_SIZE],
    pub enemy: [u8; BUFF_SIZE]
}

#[derive(Debug, PartialEq)]
pub enum GameState {
    Exploring,
    Combat,
    InMenu
}

fn get_file_bytes(filename: String, buffer: &mut [u8; BUFF_SIZE]) -> usize {
    let mut f = File::open(&filename).expect("no file found");
    let file_size = fs::metadata(&filename).expect("unable to read metadata").len();
    let mut tmp_buffer = vec![0; file_size as usize];
    f.read(&mut tmp_buffer).expect("buffer overflow");

    for (i, byte) in tmp_buffer.iter().enumerate() {
        buffer[i] = *byte;
    }

    file_size as usize
}