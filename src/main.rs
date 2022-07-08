extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

use specs::prelude::*;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG).unwrap();

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
    let middle_right_edge = (screen_width as f32 * 0.75) as u32;
    let middle_left_edge = 0;
    let middle_bottom_edge = (screen_height as f32 * 0.75) as u32;
    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    let right_screen_divider = Rect::new(
        (middle_right_edge as f32 - 2.5) as i32,
        0i32,
        5u32,
        middle_bottom_edge,
    );
    let left_screen_divider = Rect::new(
        (middle_left_edge as f32 - 2.5) as i32,
        0i32,
        5u32,
        middle_bottom_edge,
    );
    let bottom_screen_divider = Rect::new(
        0i32,
        (middle_bottom_edge as f32 - 2.5) as i32,
        middle_right_edge,
        5u32,
    );

    let texture_creator = canvas.texture_creator();
    let enemy_texture = texture_creator.load_texture("assets/zombie.png").unwrap();
    let background_texture = texture_creator.load_texture("assets/field.png").unwrap();
    let background_rect = Rect::new(
        middle_left_edge as i32,
        0,
        (screen_width as f32 * 0.75) as u32,
        middle_bottom_edge,
    );
    let enemy_rect = Rect::new(
        (screen_width as f32 * 0.333) as i32,
        (screen_height as f32 * 0.5) as i32,
        128,
        128,
    );

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

        canvas
            .copy(&background_texture, None, background_rect)
            .unwrap();
        canvas.copy(&enemy_texture, None, enemy_rect).unwrap();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(right_screen_divider).unwrap();
        canvas.fill_rect(left_screen_divider).unwrap();
        canvas.fill_rect(bottom_screen_divider).unwrap();
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
