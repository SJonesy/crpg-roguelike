extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;

use std::time::Duration;

use specs::prelude::*;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("crpg roguelite", 800, 600)
        .position_centered()
        .fullscreen_desktop()
        .resizable()
        .build()
        .unwrap();

    let screen_width = window.size().0;
    let screen_height = window.size().1;

    println!("{} {}", screen_width, screen_height);

    let top_point = Point::new((window.size().0 as f32 * 0.7) as i32, 0);
    let bottom_point = Point::new((window.size().0 as f32 * 0.7) as i32, window.size().1 as i32);

    let mut canvas = window
        .into_canvas()
        .build()
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.draw_line(top_point, bottom_point).unwrap();

    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

//        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}


