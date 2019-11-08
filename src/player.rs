use ggez::nalgebra as na;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub id: usize,
    pub position: na::Point2<f32>,
    pub velocity: na::Point2<f32>,
}


impl Player {
    
    pub fn new(id: usize) -> Player {
        Player {
            id: id,
            position: na::Point2::new(0.0, 0.0),
            velocity: na::Point2::new(0.0, 0.0),
        }
    }
}

