use std::f32::consts::PI;

pub const PLANE_SIZE: u32 = 20;
pub const BULLET_RADIUS: u32 = 1;
pub const POWERUP_RADIUS: u32 = 20;
pub const POWERUP_SPEED: f32 = 1.8;
pub const POWERUP_SPEED_BOOST: f32 = 1.5;
pub const POWERUP_BOUNCE_HEIGHT: f32 = 10.;

// currently hardcoded to the background image size
pub const WORLD_SIZE: f32 = 3000.;
pub const DELTA_TIME: f32 = 0.01;
pub const SERVER_SLEEP_DURATION: u64 = 10;

pub const MAX_SPEED: f32 = 400.;
pub const MIN_SPEED: f32 = 50.;
pub const DEFAULT_ACCELERATION: f32 = 200.;
pub const DEFAULT_HEALTH: i16 = 150;

pub const DEFAULT_AGILITY: f32 = 100.;
pub const ANGULAR_FADE: f32 = 0.9;

pub const BULLET_VELOCITY: f32 = 600.0;
pub const BULLET_DAMAGE: i16 = 10;
pub const BULLET_MAX_TRAVEL: f32 = WORLD_SIZE * 0.3;
pub const BULLET_START: f32 = 30.;
pub const PLAYER_COOLDOWN: f32 = 0.2;
pub const BULLET_ARM_TIME: f32 = 0.03;

// Time between laser charge start and fire
pub const LASER_FIRE_TIME: f32 = 0.8;
// Duration of laser damage
pub const LASER_ACTIVE_TIME: f32 = 0.02;
// Time at which the beam lingers while not doing any damage
pub const LASER_DECAY_TIME: f32 = 0.3;
// Distance at which the laser is effective
pub const LASER_RANGE: f32 = 300.;
// Damage of laser hit
pub const LASER_DAMAGE: i16 = 200;
pub const LASER_RANGE_EXTRA: u32 = 10;

pub const MISSILE_LOCK_ANGLE: f32 = PI/2.;
pub const MISSILE_KEO_P: f32 = 5.;
pub const MISSILE_SPEED: f32 = 500.;
pub const MISSILE_LIFE_TIME: f32 = 5.;

pub const WINDOW_SIZE: f32 = 700.;

pub const POWERUP_AMOUNT: u32 = 10;
pub const POWERUP_HEALTH_BOOST: i16 = 40;
pub const POWERUP_SLOWTIME_FACTOR: f32 = 3.;

pub const MINI_MAP_SIZE: f32 = 300.;

pub const PLANE_SELECTION_POS: (f32, f32) = (100., 450.);
pub const PLANE_SELECTION_SIZE: f32 = 200.;

pub const COLOR_SELECTION_POS: (f32, f32) = (400., 450.);
pub const COLOR_SELECTION_SIZE: f32 = 200.;

pub const NAME_POS: (f32, f32) = (50., 150.);

pub const PARTICLE_SPAWN_RATE: f32 = 0.05;

pub const COLLISION_DAMAGE: i16 = 40;
pub const COLLISION_GRACE_PERIOD: f32 = 1.;

pub const HIT_SEQUENCE_AMOUNT: f32 = 0.7;
pub const MAX_RED_ALPHA: f32 = 0.7;
pub const HIT_SEQUENCE_RATE: f32 = 0.015;
pub const HIT_SHAKE_SPEED: f32 = 55.;
pub const HIT_SHAKE_MAGNITUDE: f32 = 7.;

