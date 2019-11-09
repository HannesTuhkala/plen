use std::io::prelude::*;
use std::net::TcpStream;
use std::env;
use std::path;

use nalgebra::Point2;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::input::keyboard;

mod player;
mod bullet;
mod gamestate;
mod constants;

enum Action {
    Up,
    Down,
    Left,
    Right,
    //TO BE IMPLEMENTED: Shoot,
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
}

impl MainState {
    fn new(my_id: u64, stream: MessageReader<ServerMessage>)
        -> ggez::GameResult<MainState>
    {
        let s = MainState {
            server_reader: stream,
            my_id,
            game_state: gamestate::GameState::new()
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
        if keyboard::is_key_pressed(ctx, event::KeyCode::W) {
            y_input += 1.0;
        }
        if keyboard::is_key_pressed(ctx, event::KeyCode::S) {
            y_input -= 1.0;
        }

        let mut x_input = 0.0;
        if keyboard::is_key_pressed(ctx, event::KeyCode::A) {
            x_input -= 1.0;
        } 
        if keyboard::is_key_pressed(ctx, event::KeyCode::D) {
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
        .window_setup(ggez::conf::WindowSetup::default().title("Flying broccoli"))
        .add_resource_path(resource_dir)
        .build()?;

    let state = &mut MainState::new(my_id, reader)?;
    event::run(ctx, event_loop, state)
}
