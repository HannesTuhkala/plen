use nalgebra::Point2;
use nalgebra as na;
use crate::constants;

pub fn modulo(x: f32, div: f32) -> f32 {
    (x % div + div) % div
}

pub fn wrap_around(pos: Point2<f32>) -> Point2<f32> {
    Point2::new(modulo(pos.x, constants::WORLD_SIZE), modulo(pos.y, constants::WORLD_SIZE))
}

pub fn find_closest_vector_to_point(
    this: na::Point2<f32>, other: na::Point2<f32>
    ) -> na::Vector2<f32> {
    let mut res = other - this;
    let mut shortest = res.x.powi(2) + res.y.powi(2);
    for tile_x in &[-1., 0., 1.] {
        for tile_y in &[-1., 0., 1.] {
            let offset = na::Vector2::new(
                constants::WORLD_SIZE*tile_x,
                constants::WORLD_SIZE*tile_y,
            );
            let pos = other + offset;
            let center_to_point = pos - this;
            let dist = center_to_point.x.powi(2) + center_to_point.y.powi(2);
            if dist < shortest {
                res = center_to_point;
                shortest = dist;
            }
        }
    }
    res
}
