use std::io::prelude::*;
use std::net::TcpStream;

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
            player: player::Player::new(my_id, Point2::new(0., 0.)),
        };
        Ok(s)
    }

    fn get_input(ctx: &ggez::Context) -> Option<Action> {
        if keyboard::is_key_pressed(ctx, event::KeyCode::W) {
            Some(Action::Up)
        } else if keyboard::is_key_pressed(ctx, event::KeyCode::S) {
            Some(Action::Down)
        } else if keyboard::is_key_pressed(ctx, event::KeyCode::A) {
            Some(Action::Left)
        } else if keyboard::is_key_pressed(ctx, event::KeyCode::D) {
            Some(Action::Right)
        } else {
            None
        }
    }

    fn wrap_around(pos: na::Point2<f32>) -> na::Point2<f32> {
        let mut new_x = pos.x;
        let mut new_y = pos.y;

        if pos.x > constants::WORLD_SIZE {
            new_x = 0.;
        } else if pos.x < 0. {
            new_x = constants::WORLD_SIZE;
        }

        if pos.y > constants::WORLD_SIZE {
            new_y = 0.;
        } else if pos.y < 0. {
            new_y = constants::WORLD_SIZE;
        }

        na::Point2::new(new_x, new_y)
    }

    fn update_player_position(&mut self, ctx: &mut ggez::Context) {
        let mut dx = 0.;
        let mut dy = 0.;
        match MainState::get_input(ctx) {
            Some(Action::Up) => dy -= constants::DEFAULT_ACCELERATION,
            Some(Action::Down) => dy += constants::DEFAULT_ACCELERATION,
            Some(Action::Left) => dx -= constants::DEFAULT_ACCELERATION,
            Some(Action::Right) => dx += constants::DEFAULT_ACCELERATION,
            None => ()
        };

        self.player.velocity += na::Vector2::new(dx, dy);
        self.player.position = MainState::wrap_around(
            self.player.position + self.player.velocity);
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let messages = self.server_stream.fetch_bytes();
        // TODO: Use a real loop
        while let Some(message) = self.server_stream.next() {
            match message {
                ServerMessage::Ping => {}
                ServerMessage::AssignId(_) => {panic!("Got new ID after intialisatioin")}
                ServerMessage::GameState(state) => {
                    // TODO: Handle GameState
                }
            }
        }
        self.update_player_position(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.1, 0.1, 1.0].into());
        self.player.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
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
