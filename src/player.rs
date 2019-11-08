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
        let image = graphics::Image::new(ctx, "/yeehawcessna.png")?;
        graphics::draw(ctx, &image, (self.position,))?;
        Ok(())
    }
}

