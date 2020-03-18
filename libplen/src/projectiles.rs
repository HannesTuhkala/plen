use rand::Rng;

use enum_dispatch::enum_dispatch;
use serde_derive::{Serialize, Deserialize};

use crate::constants;
use crate::math::{self, Vec2};
use crate::hurricane::Hurricane;
use crate::player::Player;


#[enum_dispatch]
#[derive(Serialize, Deserialize, Clone)]
pub enum ProjectileKind {
    Bullet,
    Missile,
}

#[enum_dispatch(ProjectileKind)]
pub trait Projectile {
    fn update(&mut self, players: &[Player], delta_time: f32, hurricane: &Option<Hurricane>);
    fn is_done(&self) -> bool;
    fn is_armed(&self) -> bool;

    // Accessor functions
    fn get_id(&self) -> u64;
    fn get_shooter(&self) -> u64;
    fn get_shooter_name(&self) -> String;
    fn get_position(&self) -> Vec2;
    fn get_damage(&self) -> i16;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Bullet {
    pub id: u64,
    pub position: Vec2,
    pub velocity: Vec2,
    pub traveled_distance: f32,
    pub damage: i16,
    pub lifetime: f32,
    pub owner: u64,
    pub owner_name: String,
}

impl Bullet {
    pub fn new(position: Vec2, velocity: Vec2, damage: i16, owner: u64, owner_name: String)
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
    fn update(
        &mut self, _players: &[Player], delta_time: f32, hurricane: &Option<Hurricane>
    ) {
        hurricane.as_ref().map(|h| {
            let force = h.get_wind_force_at_position(self.position)*delta_time;
            self.velocity += force;
        });
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
    fn get_position(&self) -> Vec2 {self.position}
    fn get_damage(&self) -> i16 {self.damage}
    fn get_id(&self) -> u64 {self.id}
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LaserBeam {
    pub position: Vec2,
    pub angle: f32,
    pub damage: i16,
    pub lifetime: f32,
    pub owner: u64,
    pub owner_name: String,
}

impl LaserBeam {
    pub fn new(
        position: Vec2,
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
    pub position: Vec2,
    pub lifetime: f32,
    pub damage: i16,
    pub owner: u64,
    pub owner_name: String,
    pub speed: f32,
}

impl Missile {
    pub fn new(
        position: Vec2,
        angle: f32,
        damage: i16,
        owner: u64,
        speed: f32,
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
            speed,
        }
    }
}

impl Projectile for Missile {
    fn update(
        &mut self, players: &[Player], delta_time: f32, hurricane: &Option<Hurricane>
    ) {
        self.lifetime += delta_time;
        // Check if there are players in the line of sight of the missile
        let to_track = players.iter()
            // Don't track the shooter
            .filter(|p| p.id != self.owner)
            // Calculate the angle to the missile
            .map(|p| {
                let direction_to = p.position - self.position;
                let angle_to = math::angle_diff(
                    self.angle,
                    direction_to.angle()
                );
                (direction_to, angle_to)
            })
            .min_by_key(|(direction_to, _)| direction_to.norm() as i32);

        let target_angle_diff = if let Some((_, angle)) = to_track {
            angle
        }
        else {
            0.
        };

        self.angular_velocity +=
            constants::MISSILE_KOH_PEY
            * target_angle_diff
            * delta_time;

        self.speed += (self.speed + constants::MISSILE_ACCELERATION * delta_time)
            .min(constants::MISSILE_MAX_SPEED);
        self.angle += self.angular_velocity * delta_time;
        let new_direction = Vec2::from_direction(self.angle, 1.);
        self.position += new_direction * self.speed * delta_time;
        self.position = math::wrap_around(self.position);
    }
    fn is_armed(&self) -> bool {true}
    fn is_done(&self) -> bool {
        self.lifetime > constants::MISSILE_LIFE_TIME
    }
    fn get_shooter(&self) -> u64 {self.owner}
    fn get_shooter_name(&self) -> String {self.owner_name.clone()}
    fn get_position(&self) -> Vec2 {self.position}
    fn get_damage(&self) -> i16 {self.damage}
    fn get_id(&self) -> u64 {self.id}
}
