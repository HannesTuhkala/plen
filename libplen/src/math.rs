use nalgebra::Point2;
use crate::constants;

pub fn modulo(x: f32, div: f32) -> f32 {
    (x % div + div) % div
}

pub fn wrap_around(pos: Point2<f32>) -> Point2<f32> {
    Point2::new(modulo(pos.x, constants::WORLD_SIZE), modulo(pos.y, constants::WORLD_SIZE))
}
