use crate::{RenderState, GameState};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas};
use sdl2::image::LoadTexture;


pub fn render(
    canvas: &mut WindowCanvas,
    render_state: RenderState,
) -> Result<(), String> {
    // Clear Background
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let main_view_right_edge = (render_state.screen_width as f32 * 0.75) as u32;
    let main_view_left_edge = 0;
    let main_view_bottom_edge = (render_state.screen_height as f32 * 0.75) as u32;

    // Main Game Area
    let texture_creator = canvas.texture_creator();
    let background_texture = texture_creator.load_texture_bytes(&render_state.background).unwrap();
    let enemy_texture = texture_creator.load_texture_bytes(&render_state.enemy).unwrap();
    let background_rect = Rect::new(
        main_view_left_edge as i32,
        0,
        (render_state.screen_width as f32 * 0.75) as u32,
        main_view_bottom_edge,
    );
    let enemy_rect = Rect::new(
        (render_state.screen_width as f32 * 0.333) as i32,
        (render_state.screen_height as f32 * 0.5) as i32,
        128,
        128,
    );

    // Text Lines
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let font = ttf_context.load_font("assets/font.ttf", 16).unwrap();
    let space_between_game_area_and_text = 25;
    let line_area = render_state.screen_height - main_view_bottom_edge - space_between_game_area_and_text;
    let space_per_line = line_area / 5;
    for i in 0..5 {
        let surface = font.render("This is a line.").blended(Color::RGBA(255, 0, 0, 255)).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let target = Rect::new(
            5,
            (main_view_bottom_edge + space_between_game_area_and_text + (i * space_per_line)) as i32,
            surface.width(),
            surface.height());
        canvas.copy(&texture, None, Some(target)).unwrap();
    }

    if render_state.game_state == GameState::Combat {
        // Draw game window
        canvas
            .copy(&background_texture, None, background_rect)
            .unwrap();
        canvas.copy(&enemy_texture, None, enemy_rect).unwrap();
        canvas.set_draw_color(Color::RGB(200, 200, 200));
    }

    // Draw section dividers
    let right_screen_divider = Rect::new(
        (main_view_right_edge as f32 - 2.5) as i32,
        0i32,
        5u32,
        main_view_bottom_edge,
    );
    let left_screen_divider = Rect::new(
        (main_view_left_edge as f32 - 2.5) as i32,
        0i32,
        5u32,
        main_view_bottom_edge,
    );
    let bottom_screen_divider = Rect::new(
        0i32,
        (main_view_bottom_edge as f32 - 2.5) as i32,
        main_view_right_edge,
        5u32,
    );
    canvas.fill_rect(right_screen_divider).unwrap();
    canvas.fill_rect(left_screen_divider).unwrap();
    canvas.fill_rect(bottom_screen_divider).unwrap();

    canvas.present();
    Ok(())
}
