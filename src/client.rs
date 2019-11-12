mod player;
mod assets;
mod map;
mod bullet;
mod gamestate;
mod constants;
mod messages;
mod powerups;
mod math;
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
use ears::AudioController;

use assets::Assets;
use messages::{MessageReader, ClientMessage, ServerMessage, SoundEffect};

use menu::MenuState;

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
    let data = serde_json::to_string(msg)
        .expect("Failed to encode message");
    stream.write(data.as_bytes())
        .expect("Failed to send message to server");
    stream.write(&[0])
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
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
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
                            self.assets.powerup.play_at(pos);
                        }
                        SoundEffect::Gun => {
                            self.assets.gun.play_at(pos);
                        }
                        SoundEffect::Explosion => {
                            self.assets.explosion.play_at(pos);
                            self.map.add_explosion(pos);
                        }
                    }
                }
                ServerMessage::YouDied => {
                    ctx.continuing = false
                }
            }
        }

        ears::listener::set_position([self.camera_position.x, 0., self.camera_position.y]);

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
        );
        graphics::present(ctx)?;
        Ok(())
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
