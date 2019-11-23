use nalgebra as na;
use serde_derive::{Serialize, Deserialize};

use crate::constants;
use crate::math;
use rand::Rng;
use crate::player::Player;

use enum_dispatch::enum_dispatch;


#[enum_dispatch]
#[derive(Serialize, Deserialize, Clone)]
pub enum ProjectileKind {
    Bullet,
    Missile,
}

#[enum_dispatch(ProjectileKind)]
pub trait Projectile {
    fn update(&mut self, players: &[Player], delta_time: f32);
    fn is_done(&self) -> bool;
    fn is_armed(&self) -> bool;

    // Accessor functions
    fn get_id(&self) -> u64;
    fn get_shooter(&self) -> u64;
    fn get_shooter_name(&self) -> String;
    fn get_position(&self) -> na::Point2<f32>;
    fn get_damage(&self) -> i16;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Bullet {
    pub id: u64,
    pub position: na::Point2<f32>,
    pub velocity: na::Vector2<f32>,
    pub traveled_distance: f32,
    pub damage: i16,
    pub lifetime: f32,
    pub owner: u64,
    pub owner_name: String,
}

impl Bullet {
    pub fn new(position: na::Point2<f32>, velocity: na::Vector2<f32>, damage: i16, owner: u64, owner_name: String)
        -> Bullet
    {
        let mut rng = rand::thread_rng();
        Bullet {
            id: rng.gen_range(0, u64::max_value()),
            position,
            velocity,
            traveled_distance: 0.,
            damage,
            lifetime: 0.,
            owner,
            owner_name,
        }
    }
}

impl Projectile for Bullet {
    fn update(&mut self, _players: &[Player], delta_time: f32) {
        self.position = math::wrap_around(self.position + self.velocity * delta_time);
        self.traveled_distance += self.velocity.norm() * delta_time;
        self.lifetime += delta_time;
    }

    fn is_armed(&self) -> bool {
        self.lifetime > constants::BULLET_ARM_TIME
    }
    fn is_done(&self) -> bool {
        self.traveled_distance > constants::BULLET_MAX_TRAVEL
    }

    fn get_shooter(&self) -> u64 {self.owner}
    fn get_shooter_name(&self) -> String {self.owner_name.clone()}
    fn get_position(&self) -> na::Point2<f32> {self.position}
    fn get_damage(&self) -> i16 {self.damage}
    fn get_id(&self) -> u64 {self.id}
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LaserBeam {
    pub position: na::Point2<f32>,
    pub angle: f32,
    pub damage: i16,
    pub lifetime: f32,
    pub owner: u64,
    pub owner_name: String,
}

impl LaserBeam {
    pub fn new(
        position: na::Point2<f32>,
        angle: f32,
        damage: i16,
        owner: u64,
        owner_name: String,
    ) -> Self {
        Self {
            position,
            angle,
            damage,
            lifetime: constants::LASER_ACTIVE_TIME,
            owner,
            owner_name
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


#[derive(Serialize, Deserialize, Clone)]
pub struct Missile {
    pub id: u64,
    pub angular_velocity: f32,
    pub angle: f32,
    pub position: na::Point2<f32>,
    pub lifetime: f32,
    pub damage: i16,
    pub owner: u64,
    pub owner_name: String,
}

impl Missile {
    pub fn new(
        position: na::Point2<f32>,
        angle: f32,
        damage: i16,
        owner: u64,
        owner_name: String,
    ) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id: rng.gen_range(0, u64::max_value()),
            position,
            angular_velocity: 0.,
            angle,
            damage,
            lifetime: 0.,
            owner,
            owner_name,
        }
    }
}

impl Projectile for Missile {
    fn update(&mut self, players: &[Player], delta_time: f32) {
        let movement_direction = na::Vector2::new(
                self.angle.cos(),
                self.angle.sin(),
            );

        // Check if there are players in the line of sight of the missile
        let to_track = players.iter()
            // Don't track the shooter
            .filter(|p| p.id != self.owner)
            // Calculate the angle to the missile
            .map(|p| {
                let direction_to = (p.position - self.position).normalize();

                let angle_to = na::Rotation2::rotation_between(
                    &movement_direction,
                    &direction_to
                ).angle();

                (direction_to, angle_to)
            })
            // .filter(|(_, angle_to)| {
            //      *angle_to > constants::MISSILE_LOCK_ANGLE
            // })
            .min_by_key(|(direction_to, _)| direction_to.norm() as i32);

        let target_angle = if let Some((_, angle)) = to_track {
            angle
        }
        else {
            self.angle
            // 0.
        };

        self.angular_velocity +=
            constants::MISSILE_KEO_P
            * (target_angle - self.angle)
            * delta_time;

        self.angle += self.angular_velocity * delta_time;
        let new_direction = na::Vector2::new(
                self.angle.cos(),
                self.angle.sin(),
            );
        self.position += new_direction * constants::MISSILE_SPEED * delta_time;
    }
    fn is_armed(&self) -> bool {true}
    fn is_done(&self) -> bool {
        self.lifetime > constants::MISSILE_LIFE_TIME
    }
    fn get_shooter(&self) -> u64 {self.owner}
    fn get_shooter_name(&self) -> String {self.owner_name.clone()}
    fn get_position(&self) -> na::Point2<f32> {self.position}
    fn get_damage(&self) -> i16 {self.damage}
    fn get_id(&self) -> u64 {self.id}
}
