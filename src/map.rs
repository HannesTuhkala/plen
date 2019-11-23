use nalgebra as na;
use rand::prelude::*;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::gfx::primitives::DrawRenderer;

use libplen::player;
use libplen::constants;
use libplen::gamestate::GameState;
use libplen::projectiles::Projectile;
use crate::assets::Assets;
use crate::rendering::{
    draw_texture,
    draw_texture_centered,
    draw_texture_rotated,
    draw_texture_rotated_and_scaled
};

#[derive(Clone)]
pub struct SmokeParticle {
    alive: bool,
    alpha: f32,
    position: na::Point2<f32>,
}

impl SmokeParticle {
    fn new() -> Self {
        Self {
            alive: false,
            alpha: 0.,
            position: na::Point2::new(0., 0.),
        }
    }
}

#[derive(Clone)]
pub struct ExplosionParticle {
    alive: bool,
    alpha: f32,
    position: na::Point2<f32>,
    velocity: na::Vector2<f32>,
}

impl ExplosionParticle {
    fn new() -> Self {
        Self {
            alive: false,
            alpha: 0.,
            position: na::Point2::new(0., 0.),
            velocity: na::Vector2::new(0., 0.),
        }
    }
}

pub struct Map {
    smoke_particles: Vec<SmokeParticle>,
    explosion_particles: Vec<ExplosionParticle>,
    smoke_timer: f32,
}

impl Map {
    pub fn new() -> Map {
        Map {
            smoke_particles: vec![SmokeParticle::new(); 200],
            explosion_particles: vec![ExplosionParticle::new(); 200],
            smoke_timer: 0.
        }
    }

    pub fn add_explosion(&mut self, pos: na::Point2<f32>) {
        let mut rng = rand::thread_rng();
        self.smoke_timer = constants::PARTICLE_SPAWN_RATE;

        let mut spawned_particles = 0;
        for explosion_particle in &mut self.explosion_particles {
            if !explosion_particle.alive {
                let random_offset = na::Vector2::new(
                    (rng.gen::<f32>() - 0.5) * 5.,
                    (rng.gen::<f32>() - 0.5) * 5.,
                );

                let mut rng = rand::thread_rng();
                let angle = rng.gen::<f32>() * std::f32::consts::PI * 2.;

                explosion_particle.alive = true;
                explosion_particle.alpha = 1.0;
                explosion_particle.position = pos + random_offset;
                explosion_particle.velocity = na::Vector2::new(
                    50. * angle.cos(),
                    50. * angle.sin()
                );

                spawned_particles += 1;
                if spawned_particles >= 10 {
                    break;
                }
            }
        }
    }

    pub fn update_particles(&mut self, delta_time: f32, game_state: &GameState) {
        self.smoke_timer -= delta_time;
        if self.smoke_timer <= 0. {
            let mut rng = rand::thread_rng();
            self.smoke_timer = constants::PARTICLE_SPAWN_RATE;
            for player in &game_state.players {
                if player.is_invisible() {
                    // don't draw player if invisible
                    continue;
                }
                let random_offset = na::Vector2::new(
                    (rng.gen::<f32>() - 0.5) * 5.,
                    (rng.gen::<f32>() - 0.5) * 5.,
                );
                for smoke_particle in &mut self.smoke_particles {
                    if !smoke_particle.alive {
                        smoke_particle.alive = true;
                        smoke_particle.alpha = 1.0;
                        smoke_particle.position = player.position + random_offset;
                        break;
                    }
                }
            }
        }

        for smoke_particle in &mut self.smoke_particles {
            if smoke_particle.alive {
                smoke_particle.alpha -= delta_time;
                if smoke_particle.alpha <= 0. {
                    smoke_particle.alive = false;
                }
            }
        }

        for explosion_particle in &mut self.explosion_particles {
            if explosion_particle.alive {
                explosion_particle.position += explosion_particle.velocity * delta_time;
                explosion_particle.alpha -= delta_time;
                if explosion_particle.alpha <= 0. {
                    explosion_particle.alive = false;
                }
            }
        }
    }

    pub fn draw(
        &self,
        my_id: u64,
        canvas: &mut Canvas<Window>,
        camera_position: na::Point2<f32>,
        game_state: &GameState,
        assets: &mut Assets,
        powerup_rotation: f32,
        hit_effect_timer: f32,
    ) -> Result<(), String> {
        let screen_center = na::Vector2::new(
            constants::WINDOW_SIZE * 0.5,
            constants::WINDOW_SIZE * 0.5,
        );

        // the offset which causes the camera to shake upon getting hit
        let mut hit_offset = na::Vector2::new(0., 0.);
        if hit_effect_timer > 0. {
            hit_offset = na::Vector2::new(
                (hit_effect_timer*constants::HIT_SHAKE_SPEED*0.9).sin()
                    * constants::HIT_SHAKE_MAGNITUDE * hit_effect_timer,
                (hit_effect_timer*constants::HIT_SHAKE_SPEED*1.1).sin()
                    * constants::HIT_SHAKE_MAGNITUDE * hit_effect_timer,
            );
        }

        for tile_x in &[-1., 0., 1.] {
            for tile_y in &[-1., 0., 1.] {
                let offset = na::Vector2::new(
                    tile_x * constants::WORLD_SIZE,
                    tile_y * constants::WORLD_SIZE,
                );

                let background_position = na::Point2::new(
                    -camera_position.x,
                    -camera_position.y
                ) + offset;
                draw_texture(canvas, &assets.background, background_position)?;
            }
        }

        for tile_x in &[-1., 0., 1.] {
            for tile_y in &[-1., 0., 1.] {
                let offset = na::Vector2::new(
                    tile_x * constants::WORLD_SIZE,
                    tile_y * constants::WORLD_SIZE,
                );

                self.place_world_at(
                    canvas,
                    assets,
                    game_state,
                    camera_position + hit_offset,
                    offset,
                    powerup_rotation,
                    my_id
                )?;

                for player in &game_state.players {
                    if player.is_invisible() && my_id != player.id {
                        // don't draw player if invisible
                        continue;
                    }
                    let offset = na::Vector2::new(
                        tile_x * constants::WORLD_SIZE,
                        tile_y * constants::WORLD_SIZE,
                    );

                    let position = na::Point2::new(
                        player.position.x - camera_position.x,
                        player.position.y - camera_position.y,
                    ) + offset + hit_offset + screen_center;

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

        Self::draw_red_hit_effect(hit_effect_timer, canvas);

        if let Some(my_player) = game_state.get_player_by_id(my_id) {
            Map::draw_mini_map(game_state, canvas, assets, &my_player)?;
        }

        Self::draw_ui(my_id, game_state, canvas, assets)?;

        Self::draw_killfeed(canvas, assets, game_state)
    }

    fn draw_red_hit_effect(hit_effect_timer: f32, canvas: &mut Canvas<Window>) {
        if hit_effect_timer > 0. {
            let rect = sdl2::rect::Rect::new(
                0, 0, constants::WINDOW_SIZE as u32, constants::WINDOW_SIZE as u32,
            );

            canvas.set_draw_color((
                200, 0, 0, (hit_effect_timer/constants::HIT_SEQUENCE_AMOUNT
                    * constants::MAX_RED_ALPHA * 255.) as u8
            ));
            canvas.fill_rect(rect).unwrap();
        }
    }

    fn place_world_at(
        &self,
        canvas: &mut Canvas<Window>,
        assets: &mut Assets,
        game_state: &GameState,
        camera_position: na::Point2<f32>,
        offset: na::Vector2<f32>,
        powerup_rotation: f32,
        my_id: u64,
    ) -> Result<(), String> {
        let screen_center = na::Vector2::new(
            constants::WINDOW_SIZE * 0.5,
            constants::WINDOW_SIZE * 0.5,
        );

        for smoke_particle in &self.smoke_particles {
            let position = na::Point2::new(
                smoke_particle.position.x - camera_position.x,
                smoke_particle.position.y - camera_position.y,
            ) + offset + screen_center;
            if smoke_particle.alive {
                assets.smoke.set_alpha_mod((smoke_particle.alpha * 255.) as u8);
                draw_texture_rotated_and_scaled(
                    canvas,
                    &assets.smoke,
                    position,
                    0.,
                    na::Vector2::new(0.2, 0.2)
                )?;
                assets.smoke.set_alpha_mod(255);
            }
        }

        for explosion_particle in &self.explosion_particles {
            let position = na::Point2::new(
                explosion_particle.position.x - camera_position.x,
                explosion_particle.position.y - camera_position.y,
            ) + offset + screen_center;
            if explosion_particle.alive {
                assets.smoke.set_alpha_mod((explosion_particle.alpha * 255.) as u8);
                draw_texture_centered(canvas, &assets.smoke, position)?;
                assets.smoke.set_alpha_mod(255);
            }
        }

        for player in &game_state.players {
            if player.is_invisible() && player.id != my_id {
                // don't draw player if invisible
                continue;
            }
            let opacity = if player.is_invisible() {128} else {255};
            let position = na::Point2::new(
                player.position.x - camera_position.x,
                player.position.y - camera_position.y,
            ) + offset + screen_center;
            let texture = assets.planes.get_mut(&player.planetype).expect("Missing plane asset");

            texture.set_alpha_mod(opacity);
            draw_texture_rotated_and_scaled(
                canvas,
                texture,
                position,
                player.rotation,
                na::Vector2::new(1.0 - player.angular_velocity.abs() / 8., 1.0)
            )?;

            let nametag = assets.font.render(&player.name)
                .blended(player.color.rgba())
                .expect("Could not render text");

            let texture_creator = canvas.texture_creator();
            let nametag_texture = texture_creator.create_texture_from_surface(nametag).unwrap();
            let width = nametag_texture.query().width as f32;
            draw_texture_centered(
                canvas,
                &nametag_texture,
                na::Point2::new(position.x - width/2., position.y + 30.),
            )?;

            if let Some(p) = player.laser_charge_progress() {
                let h_offset = assets.laser_charge.query().height as f32 * 0.5;
                let laser_pos = position + na::Vector2::new(
                    player.rotation.sin() * h_offset,
                    -player.rotation.cos() * h_offset
                );
                assets.laser_charge.set_alpha_mod((p * 255.) as u8);
                draw_texture_rotated(canvas, &assets.laser_charge, laser_pos, player.rotation)?;
                assets.laser_charge.set_alpha_mod(255);
            }
        }

        for laser in &game_state.lasers {
            let position = na::Point2::new(
                laser.position.x - camera_position.x,
                laser.position.y - camera_position.y,
            ) + offset + screen_center;
            let h_offset = assets.laser_charge.query().height as f32 * 0.5;
            let laser_pos = position + na::Vector2::new(
                laser.angle.sin() * h_offset,
                -laser.angle.cos() * h_offset
            );

            if laser.is_dealing_damage() {
                draw_texture_rotated(canvas, &assets.laser_firing, laser_pos, laser.angle)?;
            }
            else {
                let decay_index =
                    ((laser.decay_progress() * assets.laser_decay.len() as f32) as usize)
                        .min(assets.laser_decay.len()-1)
                    .max(0);
                draw_texture_rotated(
                    canvas, &assets.laser_decay[decay_index], laser_pos, laser.angle
                )?;
            }
        }

        for projectile in &game_state.projectiles {
            let position = na::Point2::new(
                projectile.get_position().x - camera_position.x,
                projectile.get_position().y - camera_position.y,
            ) + offset + screen_center;
            draw_texture_centered(canvas, &assets.bullet, position)?;
        }

        for powerup in game_state.powerups.iter() {
            let position = na::Point2::new(
                powerup.position.x - camera_position.x,
                powerup.position.y - camera_position.y,
            ) + offset + screen_center;
            let mini_offset = na::Vector2::new(
                0.,
                (powerup_rotation*2.).sin() * constants::POWERUP_BOUNCE_HEIGHT
            );
            draw_texture_rotated(
                canvas,
                assets.powerups.get(&powerup.kind).unwrap(),
                position + mini_offset,
                powerup_rotation,
            )?;
        }

        Ok(())
    }

    fn draw_ui(
        my_id: u64,
        game_state: &GameState,
        canvas: &mut Canvas<Window>,
        assets: &Assets,
    ) -> Result<(), String> {
        let mut x_pos = 40.;
        let y_pos = constants::WINDOW_SIZE - 20. - constants::POWERUP_RADIUS as f32;

        if let Some(p) = game_state.get_player_by_id(my_id) {
            for powerup in p.powerups.iter() {
                draw_texture_centered(
                    canvas,
                    assets.powerups.get(&powerup.kind).expect("Missing powerup graphics"),
                    na::Point2::new(x_pos, y_pos)
                )?;
                x_pos += constants::POWERUP_RADIUS as f32 * 2.5;
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
            
            draw_texture(
                canvas,
                &kill_feed_message_texture,
                na::Point2::new(
                    constants::WINDOW_SIZE - width,
                    30. * i as f32
                ),
            )?;
        }

        Ok(())
    }

    fn draw_mini_map(
        game_state: &GameState,
        canvas: &mut Canvas<Window>,
        assets: &mut Assets,
        my_player: &player::Player,
    ) -> Result<(), String> {
        Map::draw_mini_map_background(canvas)?;

        let my_pos = my_player.position;
        let mini_map_center = na::Vector2::new(
            constants::WINDOW_SIZE - constants::MINI_MAP_SIZE * 0.5,
            constants::WINDOW_SIZE - constants::MINI_MAP_SIZE * 0.5
        );

        for tile_x in &[-1., 0., 1.] {
            for tile_y in &[-1., 0., 1.] {
                let offset = na::Vector2::new(
                    tile_x * constants::MINI_MAP_SIZE,
                    tile_y * constants::MINI_MAP_SIZE,
                );
                let scale = constants::MINI_MAP_SIZE
                    / constants::WORLD_SIZE;
                for player in &game_state.players {
                    if player.is_invisible() && my_player.id != player.id {
                        // don't draw player if invisible
                        continue;
                    }
                    let position = na::Point2::new(
                        (player.position.x - my_pos.x)*scale,
                        (player.position.y - my_pos.y)*scale,
                    );
                    let dist = ((position.x + offset.x).powi(2)
                                + (position.y + offset.y).powi(2)).sqrt();
                    if dist <= constants::MINI_MAP_SIZE/2. {
                        let (r, g, b, _) = player.color.rgba();
                        assets.miniplane.set_color_mod(r, g, b);
                        draw_texture_rotated_and_scaled(
                            canvas,
                            &assets.miniplane,
                            position + offset + mini_map_center,
                            player.rotation,
                            na::Vector2::new(0.5, 0.5)
                        )?;
                    }
                }
                for powerup in &game_state.powerups {
                    let position = na::Point2::new(
                        (powerup.position.x - my_pos.x)*scale,
                        (powerup.position.y - my_pos.y)*scale,
                    );
                    let dist = ((position.x + offset.x).powi(2)
                                + (position.y + offset.y).powi(2)).sqrt();
                    if dist <= constants::MINI_MAP_SIZE/2. {
                        let pos = position + offset + mini_map_center;
                        canvas.filled_circle(
                            pos.x as i16,
                            pos.y as i16,
                            2,
                            (128, 255, 128, 180)
                        )?;
                    }
                }
            }
        }

        Ok(())
    }

    fn draw_mini_map_background(canvas: &mut Canvas<Window>) -> Result<(), String> {
        let green_color = sdl2::pixels::Color::RGBA(0, 255, 0, 200);
        let bg_color = sdl2::pixels::Color::RGBA(0, 0, 0, 200);

        canvas.filled_circle(
            (constants::WINDOW_SIZE - constants::MINI_MAP_SIZE * 0.5) as i16,
            (constants::WINDOW_SIZE - constants::MINI_MAP_SIZE * 0.5) as i16,
            (constants::MINI_MAP_SIZE * 0.5) as i16,
            bg_color
        )?;
        canvas.aa_circle(
            (constants::WINDOW_SIZE - constants::MINI_MAP_SIZE * 0.5) as i16,
            (constants::WINDOW_SIZE - constants::MINI_MAP_SIZE * 0.5) as i16,
            (constants::MINI_MAP_SIZE * 0.45) as i16,
            green_color
        )?;
        canvas.aa_circle(
            (constants::WINDOW_SIZE - constants::MINI_MAP_SIZE * 0.5) as i16,
            (constants::WINDOW_SIZE - constants::MINI_MAP_SIZE * 0.5) as i16,
            (constants::MINI_MAP_SIZE * 0.225) as i16,
            green_color
        )?;
        canvas.hline(
            (constants::WINDOW_SIZE - constants::MINI_MAP_SIZE) as i16,
            constants::WINDOW_SIZE as i16,
            (constants::WINDOW_SIZE - constants::MINI_MAP_SIZE * 0.5) as i16,
            green_color
        )?;
        canvas.vline(
            (constants::WINDOW_SIZE - constants::MINI_MAP_SIZE * 0.5) as i16,
            (constants::WINDOW_SIZE - constants::MINI_MAP_SIZE) as i16,
            constants::WINDOW_SIZE as i16,
            green_color
        )
    }
}
