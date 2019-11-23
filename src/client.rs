mod assets;
mod map;
mod menu;

use std::io::prelude::*;
use std::net::TcpStream;
use std::env;
use std::path;
use std::time::Instant;

use ggez;
use ggez::event::{self, EventHandler};
use ggez::event::winit_event::{Event, KeyboardInput, WindowEvent, ElementState};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::input::keyboard;

use assets::Assets;
use menu::MenuState;

use libplen::messages::{MessageReader, ClientMessage, ServerMessage, SoundEffect};

use libplen::gamestate;
use libplen::constants;

struct KeyStates {
    forward: ElementState,
    back: ElementState,
    left: ElementState,
    right: ElementState,
    shooting: ElementState,
}

impl KeyStates {
    fn new() -> Self {
        KeyStates {
            forward: ElementState::Released,
            back: ElementState::Released,
            left: ElementState::Released,
            right: ElementState::Released,
            shooting: ElementState::Released,
        }
    }
}

fn send_client_message(msg: &ClientMessage, stream: &mut TcpStream) {
    let data = bincode::serialize(msg).expect("Failed to encode message");
    let length = data.len() as u16;
    stream.write(&length.to_be_bytes())
        .expect("Failed to send message length to server");
    stream.write(&data)
        .expect("Failed to send message to server");
}

struct MainState<'a> {
    my_id: u64,
    camera_position: na::Point2<f32>,
    server_reader: &'a mut MessageReader<ServerMessage>,
    game_state: gamestate::GameState,
    map: map::Map,
    assets: &'a mut Assets,
    key_states: KeyStates,
    last_time: Instant,
    powerup_rotation: f32,
    hit_effect_timer: f32,
}

struct EndState<'a> {
    assets: &'a Assets
}

impl<'a> MainState<'a> {
    fn new(my_id: u64, stream: &'a mut MessageReader<ServerMessage>, assets: &'a mut Assets)
        -> ggez::GameResult<MainState<'a>>
    {
        let s = MainState {
            server_reader: stream,
            my_id,
            camera_position: na::Point2::new(0., 0.),
            game_state: gamestate::GameState::new(),
            map: map::Map::new(),
            assets: assets,
            key_states: KeyStates::new(),
            last_time: Instant::now(),
            powerup_rotation: 0.,
            hit_effect_timer: 0.,
        };
        Ok(s)
    }
}

impl<'a> EndState<'a> {
    fn new(assets: &Assets) -> EndState {
        EndState {
            assets: assets,
        }
    }
}

impl<'a> event::EventHandler for EndState<'a> {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: keyboard::KeyCode,
        _keymod: keyboard::KeyMods,
        repeat: bool
    ) {
        if keycode == keyboard::KeyCode::Return && !repeat {
            ctx.continuing = false;
        }
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.1, 0.1, 1.0].into());
        graphics::draw(
            ctx, &self.assets.end_background,
            (na::Point2::new(
                    -constants::WINDOW_SIZE/2.,
                    -constants::WINDOW_SIZE/2.,
                    ),)).unwrap();
        graphics::present(ctx)?;
        Ok(())
    }
}

impl<'a> event::EventHandler for MainState<'a> {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.update_hit_sequence();

        let elapsed = self.last_time.elapsed();
        self.last_time = Instant::now();

        self.server_reader.fetch_bytes().unwrap();
        // TODO: Use a real loop
        while let Some(message) = self.server_reader.next() {
            match message {
                ServerMessage::AssignId(_) => {panic!("Got new ID after intialisation")}
                ServerMessage::GameState(state) => {
                    self.game_state = state
                },
                ServerMessage::PlaySound(sound, pos) => {
                    match sound {
                        SoundEffect::Powerup => {
                            sdl2::mixer::Channel::all().play(
                                &self.assets.powerup, 0
                            ).unwrap();
                        }
                        SoundEffect::Gun => {
                            sdl2::mixer::Channel::all().play(
                                &self.assets.gun, 0
                            ).unwrap();
                        }
                        SoundEffect::Explosion => {
                            sdl2::mixer::Channel::all().play(
                                &self.assets.explosion, 0
                            ).unwrap();
                            self.map.add_explosion(pos);
                        }
                    }
                }
                ServerMessage::YouDied => {
                    ctx.continuing = false
                }
                ServerMessage::PlayerHit(id) => {
                    // TODO handle if it's someone elses id, for example
                    // for sound effects and stuff
                    if id == self.my_id {
                        self.start_hit_sequence();
                    }
                }
            }
        }

        let mut y_input = 0.0;
        if self.key_states.forward == ElementState::Pressed {
            y_input += 1.0;
        }
        if self.key_states.back == ElementState::Pressed {
            y_input -= 1.0;
        }

        let mut x_input = 0.0;
        if self.key_states.left == ElementState::Pressed {
            x_input -= 1.0;
        } 
        if self.key_states.right == ElementState::Pressed {
            x_input += 1.0;
        }

        self.map.update_particles(elapsed.as_secs_f32(), &self.game_state);

        let shooting = self.key_states.shooting == ElementState::Pressed;
        let input_message = ClientMessage::Input{ x_input, y_input, shooting };
        send_client_message(&input_message, &mut self.server_reader.stream);

        self.powerup_rotation += constants::POWERUP_SPEED;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.1, 0.1, 1.0].into());

        if let Some(my_player) = self.game_state.get_player_by_id(self.my_id) {
            self.camera_position = my_player.position;
        }

        self.map.draw(
            self.my_id,
            ctx,
            self.camera_position,
            &self.game_state,
            &self.assets,
            self.powerup_rotation,
            self.hit_effect_timer,
        );
        graphics::present(ctx)?;
        Ok(())
    }
}

impl<'a> MainState<'a> {
    
    fn start_hit_sequence(&mut self) {
        self.hit_effect_timer = constants::HIT_SEQUENCE_AMOUNT;
    }

    /**
     * Updates the hit sequence timer. Returns true if the hit effect timer is 0,
     * otherwise false.
     */
    fn update_hit_sequence(&mut self) -> bool {
        if self.hit_effect_timer <= 0. {
            self.hit_effect_timer = 0.;
            true
        } else {
            self.hit_effect_timer -= constants::HIT_SEQUENCE_RATE;
            false
        }
    }

}

pub fn main() -> ggez::GameResult {
    let mut should_continue = true;
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let host = std::env::var("SERVER")
        .unwrap_or(String::from("localhost:4444"));
    let stream = TcpStream::connect(host)?;
    println!("Connected to server");

    stream.set_nonblocking(true)?;
    let mut reader = MessageReader::new(stream);

    let msg = loop {
        reader.fetch_bytes().unwrap();
        if let Some(msg) = reader.next() {
            break msg;
        }
    };

    let my_id = if let ServerMessage::AssignId(id) = msg {
        println!("Received the id {}", id);
        id
    } else {
        panic!("Expected to get an id from server")
    };

    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("super_simple", "ggez")
        .window_setup(ggez::conf::WindowSetup::default()
                      .title("plyen"))
        .window_mode(ggez::conf::WindowMode::default()
                     .dimensions(constants::WINDOW_SIZE,
                                 constants::WINDOW_SIZE))
        .add_resource_path(resource_dir)
        .build()?;

    let sdl = sdl2::init().expect("Could not initialize SDL");
    let _audio = sdl.audio().expect("Could not initialize SDL audio");
    let frequency = 44_100;
    let format = sdl2::mixer::AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = sdl2::mixer::DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).expect("Could not open SDL mixer audio");
    let _mixer_context = sdl2::mixer::init(
        sdl2::mixer::InitFlag::OGG
    ).expect("Could not initialize SDL mixer");

    // Allows 16 sounds to play simultaneously
    sdl2::mixer::allocate_channels(16);

    let mut assets = Assets::new(ctx);

    let mut color_selection = 0;
    let mut plane_selection = 0;
    
    while should_continue {
        let state = &mut MenuState::new(&assets);

        state.color_selection = color_selection;
        state.plane_selection = plane_selection;

        event::run(ctx, event_loop, state)?;

        color_selection = state.color_selection;
        plane_selection = state.plane_selection;

        ctx.continuing = true;
        send_client_message(
            &ClientMessage::JoinGame { 
                name: state.name.clone(),
                plane: state.plane.clone(),
                color: state.color.clone()
            },
            &mut reader.stream
        );

        graphics::set_screen_coordinates(
            ctx,
            graphics::Rect {
                x: -constants::WINDOW_SIZE / 2.,
                y: -constants::WINDOW_SIZE / 2.,
                w: constants::WINDOW_SIZE,
                h: constants::WINDOW_SIZE,
            }
        ).expect("Could not set screen coordinates");

        let state = &mut MainState::new(my_id, &mut reader, &mut assets)?;
        while ctx.continuing {
            // Tell the timer stuff a frame has happened.
            // Without this the FPS timer functions and such won't work.
            ctx.timer_context.tick();

            event_loop.poll_events(|event| {
                // This tells `ggez` to update it's internal states, should the event require that.
                // These include cursor position, view updating on resize, etc.
                ctx.process_event(&event);

                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            should_continue = false;
                            event::quit(ctx);
                        },
                        WindowEvent::KeyboardInput {
                            input: KeyboardInput {
                                scancode,
                                state: key_state,
                                virtual_keycode: keycode,
                                ..
                            },
                            ..
                        } => {
                            match scancode {
                                constants::SCANCODE_W => { state.key_states.forward = key_state },
                                constants::SCANCODE_S => { state.key_states.back = key_state },
                                constants::SCANCODE_A => { state.key_states.left = key_state },
                                constants::SCANCODE_D => { state.key_states.right = key_state },
                                _ => {} // Handle other key events here
                            }

                            if keycode == Some(keyboard::KeyCode::Space) {
                                state.key_states.shooting = key_state;
                            }
                        }

                        // Add other window event handling here
                        _ => {}
                    },

                    // Add other event handling here
                    _ => {}
                }
            });

            state.update(ctx)?;
            state.draw(ctx)?;
        }

        if should_continue {
            ctx.continuing = true;
            let state = &mut EndState::new(&state.assets);
            event::run(ctx, event_loop, state)?;
            ctx.continuing = true;

            graphics::set_screen_coordinates(
                ctx,
                graphics::Rect {
                    x: 0.,
                    y: 0.,
                    w: constants::WINDOW_SIZE,
                    h: constants::WINDOW_SIZE,
                }
            ).expect("Could not set screen coordinates");
        }
    }
    Ok(())
}
