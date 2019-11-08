use ggez::nalgebra as na;
use serde_derive::{Serialize, Deserialize};
use ggez::graphics;
use ggez;
use crate::constants;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub id: usize,
    pub position: na::Point2<f32>,
    pub velocity: na::Vector2<f32>,
}


impl Player {
    
    pub fn new(id: usize) -> Player {
        Player {
            id: id,
            position: na::Point2::new(0.0, 0.0),
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
}

