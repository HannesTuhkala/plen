use std::io;
use std::vec;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::collections::HashMap;
use serde_json;
use nalgebra::Point2;
use nalgebra as na;

mod messages;
mod player;
mod bullet;
mod gamestate;
mod constants;

use messages::{ClientMessage, ServerMessage, MessageReader};
use player::Player;


fn send_server_message(msg: &ServerMessage, stream: &mut TcpStream)
    -> io::Result<()>
{
    let data = serde_json::to_string(msg)
        .expect("Failed to encode message");
    stream.write_all(data.as_bytes())?;
    stream.write_all(&[0])
}

// TODO: Use modulo maybe?
fn wrap_around(pos: Point2<f32>) -> Point2<f32> {
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

    Point2::new(new_x, new_y)
}

fn update_player_position(player: &mut Player, x_input: f32, y_input: f32) {
    let mut dx = 0.;
    let mut dy = 0.;

    player.speed += y_input * constants::DEFAULT_ACCELERATION;
    if player.speed > constants::MAX_SPEED {
        player.speed = constants::MAX_SPEED;
    }
    if player.speed < constants::MIN_SPEED {
        player.speed = constants::MIN_SPEED;
    }

    let rotation = x_input * constants::DEFAULT_AGILITY;

    dx += player.speed * (player.rotation - std::f32::consts::PI/2.).cos();
    dy += player.speed * (player.rotation - std::f32::consts::PI/2.).sin();
    player.velocity = na::Vector2::new(dx, dy);

    player.position = wrap_around(
        player.position + player.velocity);

    player.rotation = player.rotation + rotation;
}

fn main() {
    let mut connections = vec!();

    let listener = TcpListener::bind("127.0.0.1:30000")
        .unwrap();

    let mut state = gamestate::GameState::new();

    listener.set_nonblocking(true).unwrap();

    println!("Listening on 127.0.0.1:30000");

    let mut next_id: u64 = 0;
    loop {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    stream.set_nonblocking(true).unwrap();
                    println!("Got new connection {}", next_id);
                    send_server_message(&ServerMessage::AssignId(next_id), &mut stream);
                    connections.push((next_id, MessageReader::<ClientMessage>::new(stream)));
                    let player = Player::new(next_id, Point2::new(10., 10.));
                    state.add_player(player);
                    next_id += 1;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented
                    // via platform-specific APIs such as epoll or IOCP
                    break;
                }
                e => {e.expect("Socket listener error");}
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(10));

        let mut clients_to_delete = vec!();
        for (id, ref mut client) in connections.iter_mut() {
            match client.fetch_bytes() {
                Ok(_) => {},
                Err(e) => {
                match e.kind() {
                    io::ErrorKind::ConnectionReset => {
                        println!("Player {} disconnected", id);
                        clients_to_delete.push(*id);
                        break;
                    }
                    e => {
                        panic!("Unhandled network issue: {:?}", e)
                    }
                }
                }
            };

            let mut player_input_x = 0.0;
            let mut player_input_y = 0.0;

            // TODO: Use a real loop
            while let Some(message) = client.next() {
                match message {
                    ClientMessage::Ping => {},
                    ClientMessage::Shoot => {},
                    ClientMessage::Input(input_x, input_y) => {
                        player_input_x = input_x;
                        player_input_y = input_y;
                    }
                }
            }

            for mut player in &mut state.players {
                if player.id == *id {
                    update_player_position(&mut player, player_input_x, player_input_y);
                }
            }

            let result = send_server_message(
                &ServerMessage::GameState(state.clone()),
                &mut client.stream
            );

            if let Err(e) = result {
                match e.kind() {
                    io::ErrorKind::ConnectionReset => {
                        println!("Player {} disconnected", id);
                        clients_to_delete.push(*id);
                        break;
                    }
                    e => {
                        panic!("Unhandled network issue: {:?}", e)
                    }
                }
            }
        }
        state.players = state.players.into_iter()
            .filter(|player| !clients_to_delete.contains(&player.id))
            .collect();
        connections = connections.into_iter()
            .filter(|(id, _)| !clients_to_delete.contains(id))
            .collect();
    }
}

