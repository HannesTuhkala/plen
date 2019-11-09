use ggez::nalgebra as na;
use serde_derive::{Serialize, Deserialize};
use ggez::graphics;
use ggez;
use crate::constants;
use crate::math;

#[derive(Serialize, Deserialize, Clone)]
pub struct Bullet {
    pub id: u64,
    pub position: na::Point2<f32>,
    pub velocity: na::Vector2<f32>,
}

impl Bullet {
    pub fn new(id: u64, position: na::Point2<f32>, velocity: na::Vector2<f32>) -> Bullet {
        Bullet {
            id: id,
            position: position,
            velocity: velocity,
        }
    }

    pub fn update(&mut self) {
        self.position = math::wrap_around(self.position + self.velocity);
    }
}
