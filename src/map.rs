use std::f32::consts::PI;
use std::time::Instant;

use rand::prelude::*;
use sdl2::render::Canvas;
use sdl2::video::Window;

use libplen::player;
use libplen::powerups::PowerUpKind;
use libplen::constants;
use libplen::gamestate::GameState;
use libplen::projectiles::{ProjectileKind, Projectile};
use libplen::math::{self, Vec2, vec2};

use crate::assets::Assets;
use crate::rendering;
use crate::hurricane;

pub struct ParticleSystem<T> {
    particles: Vec<Option<T>>,
}

impl<T> ParticleSystem<T> {
    fn new(n: usize) -> Self {
        ParticleSystem {
            particles: (0..n).map(|_| None).collect(),
        }
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = &T> + 'a {
        self.particles.iter().filter_map(|p| p.as_ref())
    }

    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut T> + 'a {
        self.particles.iter_mut().filter_map(|p| p.as_mut())
    }

    fn retain(&mut self, mut f: impl FnMut(&T) -> bool) {
        for particle_opt in &mut self.particles {
            if let Some(smoke_particle) = particle_opt {
                if !f(smoke_particle) {
                    *particle_opt = None;
                }
            }
        }
    }

    fn add_particles(&mut self, mut particles: impl Iterator<Item = T>) {
        let mut next_new = particles.next();
        if next_new.is_none() {
            return;
        }

        for particle_opt in &mut self.particles {
            if particle_opt.is_none() {
                *particle_opt = next_new;

                next_new = particles.next();
                if next_new.is_none() {
                    break;
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct SmokeParticle {
    alpha: f32,
    position: Vec2,
}

#[derive(Clone)]
pub struct ExplosionParticle {
    alpha: f32,
    position: Vec2,
    velocity: Vec2,
}

pub struct SparkParticle {
    lifetime: f32,
    position: Vec2,
    velocity: Vec2,
}

enum RadarObjectType {
    Plane { rotation: f32, color: (u8, u8, u8) },
    PowerUp
}

pub struct RadarObject {
    object_type: RadarObjectType,
    lifetime: f32,
    position: Vec2,
}

pub struct Map {
    smoke_particles: ParticleSystem<SmokeParticle>,
    explosion_particles: ParticleSystem<ExplosionParticle>,
    spark_particles: ParticleSystem<SparkParticle>,
    radar_objects: ParticleSystem<RadarObject>,
    smoke_timer: f32,
    spark_timer: f32,
    start_time: Instant,
    radar_angle: f32,
}

impl Map {
    pub fn new() -> Map {
        Map {
            smoke_particles: ParticleSystem::new(200),
            explosion_particles: ParticleSystem::new(200),
            spark_particles: ParticleSystem::new(200),
            radar_objects: ParticleSystem::new(200),
            smoke_timer: 0.,
            spark_timer: 0.,
            start_time: Instant::now(),
            radar_angle: 0.,
        }
    }

    pub fn add_explosion(&mut self, pos: Vec2) {
        let mut rng = rand::thread_rng();

        self.explosion_particles.add_particles((0..10).map(|_| {
            let random_offset = vec2(
                (rng.gen::<f32>() - 0.5) * 5.,
                (rng.gen::<f32>() - 0.5) * 5.,
            );

            let mut rng = rand::thread_rng();
            let angle = rng.gen::<f32>() * PI * 2.;

            ExplosionParticle {
                alpha: 1.0,
                position: pos + random_offset,
                velocity: Vec2::from_direction(angle, 50.),
            }
        }));
    }

    pub fn update(&mut self, delta_time: f32, game_state: &GameState, my_id: u64) {
        self.update_particles(delta_time, game_state);
        if let Some(my_player) = game_state.get_player_by_id(my_id) {
            self.update_radar(delta_time, game_state, my_player);
        }
    }

    fn update_particles(&mut self, delta_time: f32, game_state: &GameState) {
        self.smoke_timer -= delta_time;
        if self.smoke_timer <= 0. {
            let mut rng = rand::thread_rng();
            self.smoke_timer = constants::SMOKE_SPAWN_RATE;

            self.smoke_particles.add_particles(
                game_state.players.iter().filter_map(|player| {
                    if player.is_invisible() {
                        // don't show smoke behind player if invisible
                        return None;
                    }
                    let random_offset = vec2(
                        (rng.gen::<f32>() - 0.5) * 5.,
                        (rng.gen::<f32>() - 0.5) * 5.,
                    );
                    Some(SmokeParticle {
                        alpha: 1.0,
                        position: player.position + random_offset,
                    })
                })
            );
        }

        self.spark_timer -= delta_time;
        if self.spark_timer <= 0. {
            let mut rng = rand::thread_rng();
            self.spark_timer = constants::SPARK_SPAWN_RATE;

            self.spark_particles.add_particles(
                game_state.players.iter().filter_map(|player| {
                    let has_speed_boost =
                        player.has_powerup(PowerUpKind::Afterburner);

                    // Only show sparks if player is visible and has afterburner
                    if player.is_invisible() || !has_speed_boost {
                        return None;
                    }

                    let random_offset = vec2(
                        (rng.gen::<f32>() - 0.5) * 2.,
                        (rng.gen::<f32>() - 0.5) * 2.,
                    );
                    Some(SparkParticle {
                        lifetime: 0.5,
                        position: player.position,
                        velocity: -player.velocity() +
                            random_offset * constants::SPARK_SPREAD,
                    })
                })
            );
        }

        for smoke_particle in self.smoke_particles.iter_mut() {
            smoke_particle.alpha -= delta_time;
        };
        self.smoke_particles.retain(|smoke_particle| smoke_particle.alpha > 0.);

        for explosion_particle in self.explosion_particles.iter_mut() {
            explosion_particle.position += explosion_particle.velocity * delta_time;
            explosion_particle.alpha -= delta_time;
        };
        self.explosion_particles.retain(
            |explosion_particle| explosion_particle.alpha > 0.
        );

        for spark in self.spark_particles.iter_mut() {
            spark.position += spark.velocity * delta_time;
            spark.lifetime -= delta_time;
        }
        self.spark_particles.retain(|spark| spark.lifetime > 0.);
    }

    pub fn update_radar(
        &mut self,
        delta_time: f32,
        game_state: &GameState,
        my_player: &player::Player
    ) {
        let old_radar_angle = self.radar_angle;
        let radar_angle =
            (self.radar_angle + delta_time * constants::RADAR_SPEED) % (PI * 2.);
        self.radar_angle = radar_angle;

        let in_radar_range = |position: Vec2| {
            let dist_squared = position.x.powi(2) + position.y.powi(2);
            if dist_squared > (constants::MINI_MAP_SIZE/2.).powi(2) {
                return false
            }

            // atan2 gives an angle in the range (-pi, pi]
            let angle = math::modulo(position.angle(), PI*2.);

            // Angle wraparound creates some special cases
            if radar_angle >= old_radar_angle {
                old_radar_angle <= angle && angle <= radar_angle
            } else {
                old_radar_angle <= angle || angle <= radar_angle
            }
        };

        let my_pos = my_player.position;

        for tile_x in &[-1., 0., 1.] {
            for tile_y in &[-1., 0., 1.] {
                let offset = vec2(
                    tile_x * constants::MINI_MAP_SIZE,
                    tile_y * constants::MINI_MAP_SIZE,
                );
                let scale = constants::MINI_MAP_SIZE / constants::WORLD_SIZE;
                self.radar_objects.add_particles(
                    game_state.players.iter().filter_map(|player| {
                        if player.id == my_player.id || player.is_invisible() {
                            // don't draw player if invisible
                            // and my player is always drawn
                            return None;
                        }
                        let position = (player.position - my_pos)*scale + offset;
                        if in_radar_range(position) {
                            return Some(RadarObject {
                                object_type: RadarObjectType::Plane {
                                    rotation: player.rotation,
                                    color: player.color.rgb(),
                                },
                                position: position,
                                lifetime: constants::RADAR_FADEOUT_TIME,
                            });
                        }
                        None
                    })
                );
                self.radar_objects.add_particles(
                    game_state.powerups.iter().filter_map(|powerup| {
                        let position = (powerup.position - my_pos)*scale + offset;
                        if in_radar_range(position) {
                            return Some(RadarObject{
                                object_type: RadarObjectType::PowerUp,
                                position: position,
                                lifetime: constants::RADAR_FADEOUT_TIME,
                            });
                        }
                        None
                    })
                );
            }
        }

        for obj in self.radar_objects.iter_mut() {
            obj.lifetime -= delta_time;
        }
        self.radar_objects.retain(|obj| obj.lifetime > 0.);
    }

    pub fn draw(
        &self,
        my_id: u64,
        canvas: &mut Canvas<Window>,
        camera_position: Vec2,
        game_state: &GameState,
        assets: &mut Assets,
        powerup_rotation: f32,
        hit_effect_timer: f32,
        hurricane: &Option<hurricane::Hurricane>,
    ) -> Result<(), String> {
        let (screen_w, screen_h) = canvas.logical_size();
        let screen_center = vec2(
            screen_w as f32 * 0.5,
            screen_h as f32 * 0.5,
        );

        // the offset which causes the camera to shake upon getting hit
        let mut hit_offset = vec2(0., 0.);
        if hit_effect_timer > 0. {
            hit_offset = vec2(
                (hit_effect_timer*constants::HIT_SHAKE_SPEED*0.9).sin()
                    * constants::HIT_SHAKE_MAGNITUDE * hit_effect_timer,
                (hit_effect_timer*constants::HIT_SHAKE_SPEED*1.1).sin()
                    * constants::HIT_SHAKE_MAGNITUDE * hit_effect_timer,
            );
        }
        let camera_position = camera_position - hit_offset;

        for tile_x in &[-1., 0., 1.] {
            for tile_y in &[-1., 0., 1.] {
                let offset = vec2(
                    tile_x * constants::WORLD_SIZE,
                    tile_y * constants::WORLD_SIZE,
                );

                let background_position = vec2(
                    -camera_position.x,
                    -camera_position.y
                ) + offset + rendering::calculate_resolution_offset(&canvas);
                rendering::draw_texture(canvas, &assets.background, background_position)?;
            }
        }

        for tile_x in &[-1., 0., 1.] {
            for tile_y in &[-1., 0., 1.] {
                let offset = vec2(
                    tile_x * constants::WORLD_SIZE,
                    tile_y * constants::WORLD_SIZE,
                );

                let (screen_w, screen_h) = canvas.logical_size();
                let screen_center = vec2(
                    screen_w as f32 * 0.5,
                    screen_h as f32 * 0.5,
                );

                let world_to_screen_position =
                    |pos| pos - camera_position + offset + screen_center;

                self.place_world_at(
                    canvas,
                    assets,
                    game_state,
                    world_to_screen_position,
                    powerup_rotation,
                    my_id
                )?;

                for player in &game_state.players {
                    if player.is_invisible() && my_id != player.id {
                        // don't draw player if invisible
                        continue;
                    }

                    let position = world_to_screen_position(player.position);

                    let health = player.health as f32;
                    let max_health = player.planetype.health() as f32;
                    const HEALTH_BAR_WIDTH: f32 = 50.;
                    let red_rect = sdl2::rect::Rect::new(
                        (position.x - HEALTH_BAR_WIDTH / 2.) as i32,
                        (position.y + 50.) as i32,
                        HEALTH_BAR_WIDTH as u32,
                        10 as u32
                    );
                    let green_rect = sdl2::rect::Rect::new(
                        (position.x - HEALTH_BAR_WIDTH / 2.) as i32,
                        (position.y + 50.) as i32,
                        (HEALTH_BAR_WIDTH * health / max_health) as u32,
                        10 as u32
                    );

                    canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
                    canvas.fill_rect(red_rect).unwrap();

                    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 0));
                    canvas.fill_rect(green_rect).unwrap();
                }
            }
        }

        Self::draw_hurricanes_wrapped_around(hurricane, camera_position, screen_center, assets, canvas);
        Self::draw_red_hit_effect(hit_effect_timer, canvas);

        if let Some(my_player) = game_state.get_player_by_id(my_id) {
            self.draw_mini_map(canvas, assets, my_player)?;
        }

        Self::draw_ui(my_id, game_state, canvas, assets, powerup_rotation)?;
        Self::draw_killfeed(canvas, assets, game_state)?;
        Self::draw_debug_lines(canvas, &game_state.debug_lines, camera_position, screen_center)?;

        Ok(())
    }

    fn draw_hurricanes_wrapped_around(
        hurricane: &Option<hurricane::Hurricane>,
        camera_position: Vec2,
        screen_center: Vec2,
        assets: &Assets,
        canvas: &mut Canvas<Window>,
    ) {
        hurricane.as_ref().map(|h| {
            for tile_x in &[-1., 0., 1.] {
                for tile_y in &[-1., 0., 1.] {
                    let offset = vec2(
                        tile_x * constants::WORLD_SIZE,
                        tile_y * constants::WORLD_SIZE,
                    );
                    let position = vec2(
                        h.position.x - camera_position.x,
                        h.position.y - camera_position.y,
                    ) + offset + screen_center;
                    Self::draw_hurricane(h, position, canvas, assets).unwrap();
                }
            }
        });
    }

    fn draw_red_hit_effect(hit_effect_timer: f32, canvas: &mut Canvas<Window>) {
        let (screen_w, screen_h) = canvas.logical_size();
        if hit_effect_timer > 0. {
            let rect = sdl2::rect::Rect::new(
                0, 0, screen_w, screen_h,
            );

            let opacity = (hit_effect_timer/constants::HIT_SEQUENCE_AMOUNT
                    * constants::MAX_RED_ALPHA * 255.) as u8;
            canvas.set_draw_color((200, 0, 0, opacity));
            canvas.fill_rect(rect).unwrap();
        }
    }

    fn place_world_at(
        &self,
        canvas: &mut Canvas<Window>,
        assets: &mut Assets,
        game_state: &GameState,
        world_to_screen_position: impl Fn(Vec2) -> Vec2,
        powerup_rotation: f32,
        my_id: u64
    ) -> Result<(), String> {
        for smoke_particle in self.smoke_particles.iter() {
            let position = world_to_screen_position(smoke_particle.position);
            assets.smoke.set_alpha_mod((smoke_particle.alpha * 255.) as u8);
            rendering::draw_texture_rotated_and_scaled(
                canvas,
                &assets.smoke,
                position,
                0.,
                vec2(0.2, 0.2)
            )?;
            assets.smoke.set_alpha_mod(255);
        }

        for explosion_particle in self.explosion_particles.iter() {
            let position = world_to_screen_position(explosion_particle.position);
            assets.smoke.set_alpha_mod((explosion_particle.alpha * 255.) as u8);
            rendering::draw_texture_centered(canvas, &assets.smoke, position)?;
            assets.smoke.set_alpha_mod(255);
        }

        canvas.set_draw_color((255, 255, 100));
        for spark in self.spark_particles.iter() {
            let position = world_to_screen_position(spark.position);
            rendering::draw_texture_centered(canvas, &assets.spark, position)?;
        }

        for player in &game_state.players {
            if player.is_invisible() && player.id != my_id {
                // don't draw player if invisible
                continue;
            }
            let opacity = if player.is_invisible() {128} else {255};
            let position = world_to_screen_position(player.position);
            let texture = &mut assets.planes[player.planetype];

            texture.set_alpha_mod(opacity);
            rendering::draw_texture_rotated_and_scaled(
                canvas,
                &texture,
                position,
                player.rotation,
                vec2(1.0 - player.angular_velocity.abs() / 8., 1.0)
            )?;

            if player.has_powerup(PowerUpKind::Afterburner) {
                let fire_size = 32;
                let fire_frame_ms = 100;
                let fire_frame_count = 4;
                let fire_offset = 25.;
                let elapsed_time = Instant::now().duration_since(self.start_time);
                let fire_frame = (elapsed_time.as_millis() / fire_frame_ms) as i32 % fire_frame_count;
                let angle = (player.rotation / PI * 180.) as f64;
                canvas.copy_ex(
                    &assets.fire,
                    sdl2::rect::Rect::new(
                        0,
                        fire_size * fire_frame,
                        fire_size as u32,
                        fire_size as u32,
                    ),
                    sdl2::rect::Rect::new(
                        (position.x - player.rotation.sin() * fire_offset) as i32 - fire_size / 2,
                        (position.y + player.rotation.cos() * fire_offset) as i32 - fire_size / 2,
                        fire_size as u32,
                        fire_size as u32,
                    ),
                    angle + 180.,
                    None,
                    false,
                    false,
                )?;
            }

            let nametag = assets.font.render(&player.name)
                .blended(player.color.rgba())
                .expect("Could not render text");

            let texture_creator = canvas.texture_creator();
            let nametag_texture = texture_creator.create_texture_from_surface(nametag).unwrap();
            rendering::draw_texture_centered(
                canvas,
                &nametag_texture,
                vec2(position.x, position.y + 30.),
            )?;
        }

        for player in &game_state.players {
            let position = world_to_screen_position(player.position);

            if let Some(p) = player.laser_charge_progress() {
                let h_offset = assets.laser_charge.query().height as f32 * 0.5;
                let laser_pos = position + vec2(
                    player.rotation.sin() * h_offset,
                    -player.rotation.cos() * h_offset
                );
                assets.laser_charge.set_alpha_mod((p * 255.) as u8);
                rendering::draw_texture_rotated(
                    canvas, &assets.laser_charge, laser_pos, player.rotation
                )?;
                assets.laser_charge.set_alpha_mod(255);
            }
        }

        for laser in &game_state.lasers {
            let position = world_to_screen_position(laser.position);
            let h_offset = assets.laser_charge.query().height as f32 * 0.5;
            let laser_pos = position + vec2(
                laser.angle.sin() * h_offset,
                -laser.angle.cos() * h_offset
            );

            if laser.is_dealing_damage() {
                rendering::draw_texture_rotated(canvas, &assets.laser_firing, laser_pos, laser.angle)?;
            }
            else {
                let decay_index =
                    ((laser.decay_progress() * assets.laser_decay.len() as f32) as usize)
                        .min(assets.laser_decay.len()-1)
                    .max(0);
                rendering::draw_texture_rotated(
                    canvas, &assets.laser_decay[decay_index], laser_pos, laser.angle
                )?;
            }
        }

        for projectile in &game_state.projectiles {
            let position = world_to_screen_position(projectile.get_position());

            let (asset, angle) = match projectile {
                ProjectileKind::Bullet(_) => (&assets.bullet, 0.),
                ProjectileKind::Missile(m) => (&assets.missile, m.angle),
            };
            rendering::draw_texture_rotated(canvas, asset, position, angle + PI / 2.)?;
        }

        for powerup in game_state.powerups.iter() {
            let position = world_to_screen_position(powerup.position);
            let mini_offset = vec2(
                0.,
                (powerup_rotation*2.).sin() * constants::POWERUP_BOUNCE_HEIGHT
            );
            rendering::draw_texture_rotated(
                canvas,
                &assets.powerups[powerup.kind],
                position + mini_offset,
                powerup_rotation,
            )?;
        }

        Ok(())
    }

    fn draw_hurricane(
        hurricane: &hurricane::Hurricane,
        position: Vec2,
        canvas: &mut Canvas<Window>,
        assets: &Assets
    ) -> Result<(), String> {
        let scale = hurricane.size()/constants::HURRICANE_SPRITE_SIZE;
        rendering::draw_texture_rotated_and_scaled(
            canvas,
            &assets.hurricane,
            position,
            hurricane.rotation,
            vec2(scale, scale)
        )?;
        Ok(())
    }

    fn draw_ui(
        my_id: u64,
        game_state: &GameState,
        canvas: &mut Canvas<Window>,
        assets: &Assets,
        powerup_rotation: f32,
    ) -> Result<(), String> {
        let mut x_pos = 40.;
        let y_pos = canvas.logical_size().1 as f32 - 20. - constants::POWERUP_RADIUS as f32;

        if let Some(p) = game_state.get_player_by_id(my_id) {
            for powerup in p.powerups.iter() {
                rendering::draw_texture_centered(
                    canvas,
                    &assets.powerups[powerup.kind],
                    vec2(x_pos, y_pos)
                )?;
                x_pos += constants::POWERUP_RADIUS as f32 * 2.5;
            }

            if let Some(powerup) = p.available_powerup {
                let x_pos = canvas.logical_size().0 as f32 - constants::MINI_MAP_SIZE - 40.;
                let scale = 1.
                    + (((powerup_rotation*constants::AVAILABLE_POWERUP_SCALE_SPEED)
                       .sin() + 1.)/2.)*constants::AVAILABLE_POWERUP_SCALE_AMOUNT;
                rendering::draw_texture_rotated_and_scaled(
                    canvas,
                    &assets.powerups[powerup],
                    vec2(x_pos, y_pos - constants::POWERUP_RADIUS as f32 / 1.5),
                    powerup_rotation,
                    vec2(scale, scale)
                )?;

                let instruction = assets.font.render("Press E to activate")
                    .blended((255, 255, 255, 255))
                    .expect("Could not render text");

                let texture_creator = canvas.texture_creator();
                let instruction_texture =
                    texture_creator.create_texture_from_surface(instruction).unwrap();
                rendering::draw_texture_centered(
                    canvas,
                    &instruction_texture,
                    vec2(x_pos, y_pos + constants::POWERUP_RADIUS as f32),
                )?;
            }
        }
        Ok(())
    }

    fn draw_killfeed(canvas: &mut Canvas<Window>, assets: &Assets, game_state: &GameState) -> Result<(), String> {
        let mut kill_feed = game_state.killfeed.clone();
        let messages = kill_feed.get_messages().clone();

        for (i, message) in messages.iter().enumerate() {
            let kill_feed_message = assets.font.render(&message.message)
                .blended((255, 0, 0))
                .expect("Could not render text");

            let texture_creator = canvas.texture_creator();
            let kill_feed_message_texture =
                texture_creator.create_texture_from_surface(kill_feed_message).unwrap();
            let width = kill_feed_message_texture.query().width as f32;
            
            rendering::draw_texture(
                canvas,
                &kill_feed_message_texture,
                vec2(
                    canvas.logical_size().0 as f32 - width,
                    30. * i as f32
                ),
            )?;
        }

        Ok(())
    }

    fn draw_mini_map(
        &self,
        canvas: &mut Canvas<Window>,
        assets: &mut Assets,
        my_player: &player::Player,
    ) -> Result<(), String> {
        let (screen_w, screen_h) = canvas.logical_size();
        rendering::draw_texture(
            canvas,
            &assets.minimap_background,
            vec2(
                screen_w as f32 - constants::MINI_MAP_SIZE,
                screen_h as f32 - constants::MINI_MAP_SIZE,
            )
        )?;

        let mini_map_center = vec2(
            screen_w as f32 - constants::MINI_MAP_SIZE * 0.5,
            screen_h as f32 - constants::MINI_MAP_SIZE * 0.5
        );

        for radar_object in self.radar_objects.iter() {
            let pos = mini_map_center + radar_object.position;
            let alpha =
                (radar_object.lifetime / constants::RADAR_FADEOUT_TIME * 255.) as u8;

            match radar_object.object_type {
                RadarObjectType::Plane { rotation, color } => {
                    let (r, g, b) = color;
                    assets.miniplane.set_color_mod(r, g, b);
                    assets.miniplane.set_alpha_mod(alpha);
                    rendering::draw_texture_rotated_and_scaled(
                        canvas,
                        &assets.miniplane,
                        pos,
                        rotation,
                        vec2(0.5, 0.5)
                    )?;
                    assets.miniplane.set_alpha_mod(255);
                }
                RadarObjectType::PowerUp => {
                    assets.minimap_powerup.set_alpha_mod(alpha);
                    rendering::draw_texture_centered(
                        canvas, &assets.minimap_powerup, pos
                    )?;
                    assets.minimap_powerup.set_alpha_mod(255);
                }
            }
        }

        // Draw radar line
        let mini_map_edge = mini_map_center + vec2(
            self.radar_angle.cos(),
            self.radar_angle.sin()
        ) * constants::MINI_MAP_SIZE/2.;
        canvas.draw_line(
            (mini_map_center.x as i32, mini_map_center.y as i32),
            (mini_map_edge.x as i32, mini_map_edge.y as i32)
        )?;

        // Draw my plane
        let (r, g, b) = my_player.color.rgb();
        assets.miniplane.set_color_mod(r, g, b);
        rendering::draw_texture_rotated_and_scaled(
            canvas,
            &assets.miniplane,
            mini_map_center,
            my_player.rotation,
            vec2(0.5, 0.5)
        )?;

        Ok(())
    }

    fn draw_debug_lines(
        canvas: &mut Canvas<Window>,
        lines: &[libplen::debug::DebugLine],
        camera_position: Vec2,
        screen_center: Vec2,
    ) -> Result<(), String> {
        for tile_x in &[-1., 0., 1.] {
            for tile_y in &[-1., 0., 1.] {
                for line in lines {
                    let offset = vec2(*tile_x, *tile_y) * constants::WORLD_SIZE
                        - camera_position + screen_center;
                    let start = line.start + offset;
                    let end = line.end + offset;
                    canvas.set_draw_color(line.color);
                    canvas.draw_line(
                        sdl2::rect::Point::new(start.x as i32, start.y as i32),
                        sdl2::rect::Point::new(end.x as i32, end.y as i32),
                    )?;
                }
            }
        }
        Ok(())
    }
}
