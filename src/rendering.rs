use nalgebra as na;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

pub fn draw_texture(
    canvas: &mut Canvas<Window>, texture: &Texture, pos: na::Point2<f32>
) -> Result<(), String> {
    let texture_query = texture.query();
    canvas.copy(texture, None, sdl2::rect::Rect::new(
        pos.x as i32, pos.y as i32, texture_query.width as u32, texture_query.height as u32
    ))
}

pub fn draw_texture_centered(
    canvas: &mut Canvas<Window>, texture: &Texture, pos: na::Point2<f32>
) -> Result<(), String> {
    let texture_query = texture.query();
    let w = texture_query.width;
    let h = texture_query.height;
    canvas.copy(texture, None, Some(sdl2::rect::Rect::new(
        pos.x as i32 - w as i32 / 2,
        pos.y as i32 - h as i32 / 2,
        w,
        h
    )))
}

pub fn draw_texture_rotated(
    canvas: &mut Canvas<Window>, texture: &Texture, pos: na::Point2<f32>, angle: f32
) -> Result<(), String> {
    let texture_query = texture.query();
    let w = texture_query.width as u32;
    let h = texture_query.height as u32;
    let dest_rect = sdl2::rect::Rect::new(
        pos.x as i32 - w as i32 / 2,
        pos.y as i32 - h as i32 / 2,
        w,
        h
    );
    let angle = (angle / std::f32::consts::PI * 180.) as f64;
    canvas.copy_ex(texture, None, Some(dest_rect), angle, None, false, false)
}

pub fn draw_texture_rotated_and_scaled(
    canvas: &mut Canvas<Window>,
    texture: &Texture,
    pos: na::Point2<f32>,
    angle: f32,
    scale: na::Vector2<f32>
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
    let angle = (angle / std::f32::consts::PI * 180.) as f64;
    canvas.copy_ex(texture, None, Some(dest_rect), angle, None, false, false)
}
