extern crate rand;

use ggez::nalgebra as na;
use serde_derive::{Serialize, Deserialize};
use ggez::graphics;
use ggez;
use crate::constants;
use crate::bullet;
use crate::assets::Assets;
use crate::math;

use crate::powerups::PowerUpKind;

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: u64,
    pub rotation: f32,
    pub angular_velocity: f32,
    pub speed: f32,
    pub health: u8,
    pub position: na::Point2<f32>,
    pub velocity: na::Vector2<f32>,
    pub powerups: Vec<PowerUpKind>,
    pub cooldown: f32,
}


impl Player {
    pub fn new(id: u64, position: na::Point2<f32>) -> Player {
        Player {
            id: id,
            rotation: 0.,
            angular_velocity: 0.,
            speed: 0.,
            health: 100,
            position: na::Point2::new(100.0, 100.0),
            velocity: na::Vector2::new(0.0, 0.0),
            powerups: vec!(),
            cooldown: 0.,
        }
    }

    pub fn update(&mut self, x_input: f32, y_input: f32, delta_time: f32) {
        self.cooldown = (self.cooldown - delta_time).max(0.);

        let mut dx = 0.;
        let mut dy = 0.;

        self.speed += y_input * constants::DEFAULT_ACCELERATION * delta_time;
        if self.speed > constants::MAX_SPEED {
            self.speed = constants::MAX_SPEED;
        }
        if self.speed < constants::MIN_SPEED {
            self.speed = constants::MIN_SPEED;
        }

        dx += self.speed * (self.rotation - std::f32::consts::PI/2.).cos();
        dy += self.speed * (self.rotation - std::f32::consts::PI/2.).sin();
        self.velocity = na::Vector2::new(dx, dy);

        self.position = math::wrap_around(
            self.position + self.velocity * delta_time
        );

        let angular_acceleration = x_input * constants::DEFAULT_AGILITY/10.;
        self.angular_velocity += angular_acceleration;
        self.angular_velocity *= constants::ANGULAR_FADE;
        if self.angular_velocity > constants::DEFAULT_AGILITY {
            self.angular_velocity = constants::DEFAULT_AGILITY;
        } else if self.angular_velocity < -constants::DEFAULT_AGILITY {
            self.angular_velocity = -constants::DEFAULT_AGILITY;
        }
        self.rotation = self.rotation + self.angular_velocity;
    }

    pub fn shoot(&mut self) -> Option<bullet::Bullet> {
        let dir = self.rotation - std::f32::consts::PI / 2.;

        if self.cooldown <= 0. {
            self.cooldown = constants::PLAYER_COOLDOWN;
            Some(bullet::Bullet::new(
                self.position,
                self.velocity + na::Vector2::new(
                    dir.cos() * constants::BULLET_VELOCITY,
                    dir.sin() * constants::BULLET_VELOCITY,
                ),
            ))
        } else {
            None
        }
    }

    pub fn apply_powerup(&mut self, kind: PowerUpKind) {
        self.powerups.push(kind)
    }
}
