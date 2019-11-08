use std::io;
use std::vec;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::collections::HashMap;
use serde_json;

mod messages;
mod player;
mod bullet;
mod gamestate;
mod constants;

use messages::Message;
use player::Player;


fn main() {
    let mut connections = vec!();

    let mut listener = TcpListener::bind("127.0.0.1:30000")
        .unwrap();

    listener.set_nonblocking(true);

    println!("Listening on 127.0.0.1:30000");

    let mut players = vec::Vec::<Player>::new();
    let mut next_id: u64 = 0;
    loop {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Got new connection {}", next_id);
                    connections.push((next_id, stream));
                    let mut player = Player::new(next_id);
                    players.push(player);
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

        for (id, ref mut client) in connections.iter_mut() {
            client.write(serde_json::to_string(&Message::Ping).expect("Failed to encode message").as_bytes())
                .expect("Failed to send message to client");
        }
    }
}

