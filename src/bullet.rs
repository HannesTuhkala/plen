use ggez::nalgebra as na;
use serde_derive::{Serialize, Deserialize};
use ggez::graphics;
use ggez;
use crate::constants;
use crate::math;
use rand::Rng;

#[derive(Serialize, Deserialize, Clone)]
pub struct Bullet {
    pub id: u64,
    pub position: na::Point2<f32>,
    pub velocity: na::Vector2<f32>,
    pub traveled_distance: f32,
}

impl Bullet {
    pub fn new(position: na::Point2<f32>, velocity: na::Vector2<f32>) -> Bullet {
        let mut rng = rand::thread_rng();
        Bullet {
            id: rng.gen_range(0, u64::max_value()),
            position: position,
            velocity: velocity,
            traveled_distance: 0.,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.position = math::wrap_around(self.position + self.velocity * delta_time);
        self.traveled_distance += self.velocity.norm() * delta_time;
    }
}
