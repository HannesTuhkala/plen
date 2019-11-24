use serde_derive::{Serialize, Deserialize};
use nalgebra as na;

use crate::constants;

#[derive(Serialize, Deserialize, Clone)]
enum HurricaneStatus { 
    Growing,
    Sustaining(f32), // time left
    Shrinking,
    Dead,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Hurricane {
    pub position: na::Point2<f32>,
    pub velocity: na::Vector2<f32>,
    pub strength: f32,
    size: f32,
    status: HurricaneStatus,
}

impl Hurricane {
    
    pub fn new(position: na::Point2<f32>, velocity: na::Vector2<f32>) -> Self {
        Hurricane {
            position,
            velocity,
            size: 0.,
            strength: 0.,
            status: HurricaneStatus::Growing,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.position += self.velocity*delta;
        match &mut self.status {
            HurricaneStatus::Growing => {
                self.size += constants::HURRICANE_GROW_SPEED*delta;
                if self.size >= 1. {
                    self.size = 1.;
                    self.status = HurricaneStatus::Sustaining(
                        constants::HURRICANE_SUSTAIN_TIME
                    );
                }
            }
            HurricaneStatus::Sustaining(time_left) => {
                *time_left -= delta;
                if *time_left <= 0. {
                    self.status = HurricaneStatus::Shrinking;
                }
            }
            HurricaneStatus::Shrinking => {
                self.size -= constants::HURRICANE_GROW_SPEED*delta;
                if self.size <= 0. {
                    self.size = 0.;
                    self.status = HurricaneStatus::Dead;
                }
            }
            _ => ()
        }
    }

    pub fn size(&self) -> f32 {
        self.size * constants::HURRICANE_MAX_SIZE
    }

    pub fn get_wind_force_at_position(&self, position: na::Point2<f32>) -> na::Vector2<f32> {
        // TODO implement
        na::Vector2::new(0., 0.)
        
    }

    pub fn is_dead(&self) -> bool {
        match self.status {
            HurricaneStatus::Dead => true,
            _ => false
        }
    }
}
