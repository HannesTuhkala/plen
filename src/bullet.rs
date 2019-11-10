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
    pub damage: i16,
    pub lifetime: f32,
}

impl Bullet {
    pub fn new(position: na::Point2<f32>, velocity: na::Vector2<f32>, damage: i16)
        -> Bullet
    {
        let mut rng = rand::thread_rng();
        Bullet {
            id: rng.gen_range(0, u64::max_value()),
            position: position,
            velocity: velocity,
            traveled_distance: 0.,
            damage,
            lifetime: 0.,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.position = math::wrap_around(self.position + self.velocity * delta_time);
        self.traveled_distance += self.velocity.norm() * delta_time;
        self.lifetime += delta_time;
    }

    pub fn is_armed(&mut self) -> bool {
        self.lifetime > constants::BULLET_ARM_TIME
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LaserBeam {
    pub position: na::Point2<f32>,
    pub angle: f32,
    pub damage: i16,
    pub lifetime: f32,
}

impl LaserBeam {
    pub fn new(position: na::Point2<f32>, angle: f32, damage: i16) -> Self {
        Self {
            position,
            angle,
            damage,
            lifetime: constants::LASER_ACTIVE_TIME,
        }
    }

    // Update the laser, return true if it should still be active
    pub fn update(&mut self, delta_time: f32) {
        self.lifetime -= delta_time;
    }

    pub fn is_dealing_damage(&self) -> bool {
        self.lifetime > 0.
    }
    pub fn decay_progress(&self) -> f32 {
        self.lifetime / -constants::LASER_DECAY_TIME
    }
    pub fn should_be_removed(&self) -> bool {
        self.lifetime < -constants::LASER_DECAY_TIME
    }
}
