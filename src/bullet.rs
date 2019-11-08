use ggez::nalgebra as na;
use serde_derive::{Serialize, Deserialize};
use ggez::graphics;
use ggez;
use crate::constants;

#[derive(Serialize, Deserialize)]
pub struct Bullet {
    pub id: usize,
    pub position: na::Point2<f32>,
    pub velocity: na::Point2<f32>,
}

impl Bullet {
    pub fn new(id: usize) -> Bullet {
        Bullet {
            id: id,
            position: na::Point2::new(0.0, 0.0),
            velocity: na::Point2::new(0.0, 0.0),
        }
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0, 0.0),
            constants::BULLET_RADIUS as f32,
            0.1,
            graphics::WHITE,
        )?;

        graphics::draw(ctx, &circle, (self.position,))?;
        Ok(())
    }
}
