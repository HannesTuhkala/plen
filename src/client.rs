use std::io::prelude::*;
use std::net::TcpStream;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::input::keyboard;

mod player;
mod bullet;
mod gamestate;
mod constants;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

mod messages;
use messages::{MessageReader, ServerMessage};

struct MainState {
    my_id: u64,
    server_stream: MessageReader<ServerMessage>,
    player: player::Player
}

impl MainState {
    fn new(my_id: u64, stream: MessageReader<ServerMessage>)
        -> ggez::GameResult<MainState>
    {
        let s = MainState {
            server_stream: stream,
            my_id,
            player: player::Player::new(my_id),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        update_player_position(ctx, &mut self.player);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.1, 0.1, 1.0].into());
        self.player.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn update_player_position(ctx: &mut ggez::Context,
                          player: &mut player::Player) {
    let mut dx = 0.;
    let mut dy = 0.;
    match get_input(ctx) {
        Some(Direction::Up) => dy -= 1.,
        Some(Direction::Down) => dy += 1.,
        Some(Direction::Left) => dx -= 1.,
        Some(Direction::Right) => dx += 1.,
        None => ()
    };
    
    player.velocity += na::Vector2::new(dx, dy);
    player.position += player.velocity;
}

fn get_input(ctx: &ggez::Context) -> Option<Direction> {
    if keyboard::is_key_pressed(ctx, event::KeyCode::Up) {
        Some(Direction::Up)
    } else if keyboard::is_key_pressed(ctx, event::KeyCode::Down) {
        Some(Direction::Down)
    } else if keyboard::is_key_pressed(ctx, event::KeyCode::Left) {
        Some(Direction::Left)
    } else if keyboard::is_key_pressed(ctx, event::KeyCode::Right) {
        Some(Direction::Right)
    } else {
        None
    }
}

pub fn main() -> ggez::GameResult { 
    let mut stream = TcpStream::connect("127.0.0.1:30000")?;
    println!("Connected to server");

    stream.set_nonblocking(true)?;
    let mut reader = MessageReader::new(stream);


    let msg = loop {
        reader.fetch_bytes();
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

    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(my_id, reader)?;
    event::run(ctx, event_loop, state)
}
