extern crate rand;

use ggez::nalgebra as na;
use serde_derive::{Serialize, Deserialize};
use ggez::graphics;
use ggez;
use crate::constants;
use crate::bullet;
use crate::assets::Assets;
use rand::Rng;

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: u64,
    pub rotation: f32,
    pub speed: f32,
    pub position: na::Point2<f32>,
    pub velocity: na::Vector2<f32>,
}


impl Player {
    pub fn new(id: u64, position: na::Point2<f32>) -> Player {
        Player {
            id: id,
            rotation: 0.,
            speed: 0.,
            position: na::Point2::new(100.0, 100.0),
            velocity: na::Vector2::new(0.0, 0.0),
        }
    }

    //pub fn draw(&self, ctx: &mut ggez::Context,
    //            position: na::Point2<f32>,
    //            rotation: f32,
    //            assets: &Assets) -> ggez::GameResult {
    //    graphics::draw(ctx, &assets.cessna, graphics::DrawParam::default()
    //                   .dest(position)
    //                   .rotation(rotation)
    //                   .offset(na::Point2::new(0.5, 0.5)))?;
    //    Ok(())
    //}

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
