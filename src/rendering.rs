use std::f32::consts::PI;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use libplen::constants;
use libplen::math::{Vec2, vec2};

pub fn draw_texture(
    canvas: &mut Canvas<Window>, texture: &Texture, pos: Vec2
) -> Result<(), String> {
    let texture_query = texture.query();
    canvas.copy(texture, None, Rect::new(
        pos.x as i32, pos.y as i32, texture_query.width as u32, texture_query.height as u32
    ))
}

pub fn draw_texture_centered(
    canvas: &mut Canvas<Window>, texture: &Texture, pos: Vec2
) -> Result<(), String> {
    let texture_query = texture.query();
    let w = texture_query.width;
    let h = texture_query.height;
    canvas.copy(texture, None, Rect::new(
        pos.x as i32 - w as i32 / 2,
        pos.y as i32 - h as i32 / 2,
        w,
        h
    ))
}

pub fn draw_texture_rotated(
    canvas: &mut Canvas<Window>, texture: &Texture, pos: Vec2, angle: f32
) -> Result<(), String> {
    let texture_query = texture.query();
    let w = texture_query.width as u32;
    let h = texture_query.height as u32;
    let dest_rect = Rect::new(
        pos.x as i32 - w as i32 / 2,
        pos.y as i32 - h as i32 / 2,
        w,
        h
    );
    let angle = (angle / PI * 180.) as f64;
    canvas.copy_ex(texture, None, dest_rect, angle, None, false, false)
}

pub fn draw_texture_rotated_and_scaled(
    canvas: &mut Canvas<Window>,
    texture: &Texture,
    pos: Vec2,
    angle: f32,
    scale: Vec2
) -> Result<(), String> {
    let texture_query = texture.query();
    let w = texture_query.width as f32 * scale.x;
    let h = texture_query.height as f32 * scale.y;
    let dest_rect = sdl2::rect::Rect::new(
        pos.x as i32 - w as i32 / 2,
        pos.y as i32 - h as i32 / 2,
        w as u32,
        h as u32
    );
    let angle = (angle / PI * 180.) as f64;
    canvas.copy_ex(texture, None, dest_rect, angle, None, false, false)
}

pub fn setup_coordinates(canvas: &mut Canvas<Window>) -> Result<(), String> {
    let (window_width, window_height) = canvas.window().size();
    let (w, h) = if window_width < window_height {
        (
            constants::WINDOW_SIZE,
            window_height as f32 / window_width as f32 * constants::WINDOW_SIZE
        )
    } else {
        (
            window_width as f32 / window_height as f32 * constants::WINDOW_SIZE,
            constants::WINDOW_SIZE
        )
    };

    canvas.set_logical_size(w as u32, h as u32).map_err(|e| e.to_string())
}

pub fn calculate_resolution_offset(canvas: &Canvas<Window>) -> Vec2 {
    let (w, h) = canvas.logical_size();
    let (x, y) = if w > h {
        ((w as f32 - constants::WINDOW_SIZE) * 0.5, 0.)
    } else {
        (0., (h as f32 - constants::WINDOW_SIZE) * 0.5)
    };

    vec2(x, y)
}
