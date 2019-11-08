extern crate rand;

use ggez::nalgebra as na;
use serde_derive::{Serialize, Deserialize};
use ggez::graphics;
use ggez;
use crate::constants;
use crate::bullet;
use rand::Rng;

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: u64,
    pub position: na::Point2<f32>,
    pub velocity: na::Vector2<f32>,
}


impl Player {
    pub fn new(id: u64, position: na::Point2<f32>) -> Player {
        Player {
            id: id,
            position,
            velocity: na::Vector2::new(0.0, 0.0),
        }
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                0.,
                0.,
                constants::PLANE_SIZE as f32,
                constants::PLANE_SIZE as f32,
            ),
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &rect, (self.position,))?;
        Ok(())
    }

    pub fn shoot(&self, angle: f32) -> bullet::Bullet {
        let mut rng = rand::thread_rng();

        bullet::Bullet {
            id: rng.gen_range(0, u64::max_value()),
            position: self.position,
            velocity: na::Point2::new(
                angle.cos() * constants::BULLET_VELOCITY_FACTOR + constants::BULLET_VELOCITY_CONSTANT,
                angle.sin() * constants::BULLET_VELOCITY_FACTOR + constants::BULLET_VELOCITY_CONSTANT),
        }
    }
}

