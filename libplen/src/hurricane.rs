use serde_derive::{Serialize, Deserialize};

use crate::constants;
use crate::math::{self, Vec2, vec2};

#[derive(Serialize, Deserialize, Clone)]
enum HurricaneStatus { 
    Growing,
    Sustaining(f32), // time left
    Shrinking,
    Dead,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Hurricane {
    pub position: Vec2,
    pub velocity: Vec2,
    pub rotation: f32,
    size: f32,
    status: HurricaneStatus,
}

impl Hurricane {
    
    pub fn new(position: Vec2, velocity: Vec2) -> Self {
        Hurricane {
            position,
            velocity,
            size: 0.,
            rotation: 0.,
            status: HurricaneStatus::Growing,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.position = math::wrap_around(self.position + self.velocity*delta);
        self.rotation += constants::HURRICANE_ROTATION_SPEED*delta;
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

    pub fn get_wind_force_at_position(&self, position: Vec2) -> Vec2 {
        let center_to_point = math::find_closest_vector_to_point(
            self.position, position);
        let dist = (center_to_point.x.powi(2) + center_to_point.y.powi(2)).sqrt();

        if dist < constants::HURRICANE_EYE_SIZE || dist >= self.size()/2. {
            vec2(0., 0.)
        } else {
            let normal = vec2(-center_to_point.y, center_to_point.x);
            let unit_vector = normal / dist;

            unit_vector * constants::HURRICANE_MAX_WINDSPEED *
                (self.size()/2. - dist)/(constants::HURRICANE_MAX_SIZE/2.)
        }
    }

    pub fn is_dead(&self) -> bool {
        match self.status {
            HurricaneStatus::Dead => true,
            _ => false
        }
    }
}
