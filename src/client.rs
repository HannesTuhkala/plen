use std::io::prelude::*;
use std::net::TcpStream;
use std::env;
use std::path;

use ggez;
use ggez::event::{self, EventHandler};
use ggez::event::winit_event::{Event, KeyboardInput, WindowEvent, ElementState};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::input::keyboard;

mod player;
mod map;
mod bullet;
mod gamestate;
mod constants;

struct KeyStates {
    forward: ElementState,
    back: ElementState,
    left: ElementState,
    right: ElementState,
}

impl KeyStates {
    fn new() -> Self {
        KeyStates {
            forward: ElementState::Released,
            back: ElementState::Released,
            left: ElementState::Released,
            right: ElementState::Released,
        }
    }
}

mod messages;
use messages::{MessageReader, ClientMessage, ServerMessage};

fn send_client_message(msg: &ClientMessage, stream: &mut TcpStream) {
    let data = serde_json::to_string(msg)
        .expect("Failed to encode message");
    stream.write(data.as_bytes())
        .expect("Failed to send message to server");
    stream.write(&[0])
        .expect("Failed to send message to server");
}

struct MainState {
    my_id: u64,
    server_reader: MessageReader<ServerMessage>,
    game_state: gamestate::GameState,
    key_states: KeyStates,
}

impl MainState {
    fn new(my_id: u64, stream: MessageReader<ServerMessage>)
        -> ggez::GameResult<MainState>
    {
        let s = MainState {
            server_reader: stream,
            my_id,
            game_state: gamestate::GameState::new(),
            key_states: KeyStates::new(),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.server_reader.fetch_bytes().unwrap();
        // TODO: Use a real loop
        while let Some(message) = self.server_reader.next() {
            match message {
                ServerMessage::Ping => {}
                ServerMessage::AssignId(_) => {panic!("Got new ID after intialisation")}
                ServerMessage::GameState(state) => {
                    self.game_state = state
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

        send_client_message(&ClientMessage::Input(x_input, y_input), &mut self.server_reader.stream);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.1, 0.1, 1.0].into());
        for player in &mut self.game_state.players {
            player.draw(ctx)?;
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let stream = TcpStream::connect("127.0.0.1:30000")?;
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
                      .title("Flying broccoli"))
        .window_mode(ggez::conf::WindowMode::default()
                     .dimensions(constants::WINDOW_SIZE,
                                  constants::WINDOW_SIZE))
        .add_resource_path(resource_dir)
        .build()?;

    let state = &mut MainState::new(my_id, reader)?;
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
                    WindowEvent::CloseRequested => event::quit(ctx),
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput {
                            scancode,
                            state: key_state,
                            ..
                        },
                        ..
                    } => match scancode {
                        constants::SCANCODE_W => { state.key_states.forward = key_state },
                        constants::SCANCODE_S => { state.key_states.back = key_state },
                        constants::SCANCODE_A => { state.key_states.left = key_state },
                        constants::SCANCODE_D => { state.key_states.right = key_state },
                        _ => {} // Handle other key events here
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
    Ok(())
}
