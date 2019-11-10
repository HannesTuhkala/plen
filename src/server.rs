mod messages;
mod assets;
mod player;
mod bullet;
mod gamestate;
mod constants;
mod math;
mod powerups;

use std::io;
use std::vec;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use serde_json;
use nalgebra::Point2;
use nalgebra as na;
use std::time::Instant;
use rand::Rng;

use messages::{ClientMessage, ServerMessage, MessageReader, SoundEffect};
use player::Player;
use powerups::PowerUpKind;


fn send_server_message(msg: &ServerMessage, stream: &mut TcpStream)
    -> io::Result<()>
{
    let mut data = serde_json::to_string(msg)
        .expect("Failed to encode message");
    data.push(0 as char);
    let bytes = data.as_bytes();
    let mut start = 0;
    loop {
        match stream.write(&bytes[start..data.len()]) {
            Ok(n) => {
                if n < data.len() - start {
                    start = start + n;
                }
                else {
                    break Ok(())
                }
            }
            Err(e) => {
                 match e.kind() {
                     io::ErrorKind::WouldBlock => continue,
                     io::ErrorKind::Interrupted => continue,
                     _ => return Err(e)
                 }
            }
        }
    }
}

struct Server {
    listener: TcpListener,
    connections: Vec<(u64, MessageReader<ClientMessage>)>,
    state: gamestate::GameState,
    next_id: u64,
    last_time: Instant,
}

impl Server {
    pub fn new() -> Self {
        let listener = TcpListener::bind("0.0.0.0:4444")
            .unwrap();

        listener.set_nonblocking(true).unwrap();

        println!("Listening on 0.0.0.0:4444");

        Self {
            listener,
            connections: vec!(),
            next_id: 0,
            last_time: Instant::now(),
            state: gamestate::GameState::new()
        }
    }

    pub fn update(&mut self) {
        let elapsed = self.last_time.elapsed();
        let delta_time = 1./100.;
        let dt_duration = std::time::Duration::from_millis(10);
        if elapsed < dt_duration {
            std::thread::sleep(dt_duration - elapsed);
        }
        self.last_time = Instant::now();

        self.state.update(delta_time);
        self.accept_new_connections();
        self.update_clients(delta_time);

        for bullet in &mut self.state.bullets {
            bullet.update(delta_time);
        }

        self.state.bullets.retain(
            |bullet| bullet.traveled_distance < constants::BULLET_MAX_TRAVEL
        );
    }

    fn accept_new_connections(&mut self) {
        // Read data from clients
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    stream.set_nonblocking(true).unwrap();
                    println!("Got new connection {}", self.next_id);
                    if let Err(_) = send_server_message(
                        &ServerMessage::AssignId(self.next_id),
                        &mut stream
                    ) {
                        println!("Could not send assign id message");
                        continue;
                    }
                    self.connections.push((
                        self.next_id,
                        MessageReader::<ClientMessage>::new(stream)
                    ));
                    self.next_id += 1;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented
                    // via platform-specific APIs such as epoll or IOCP
                    break;
                }
                e => {e.expect("Socket listener error");}
            }
        }
    }

    fn update_clients(&mut self, delta_time: f32) {
        // Send data to clients
        let mut clients_to_delete = vec!();
        let mut sounds_to_play = vec!();

        macro_rules! remove_player_on_disconnect {
            ($op:expr, $id:expr) => {
                match $op {
                    Ok(_) => {},
                    Err(e) => {
                        match e.kind() {
                            io::ErrorKind::ConnectionReset | io::ErrorKind::BrokenPipe => {
                                println!("Player {} disconnected", $id);
                                clients_to_delete.push($id);
                                break;
                            }
                            e => {
                                panic!("Unhandled network issue: {:?}", e)
                            }
                        }
                    }
                };
            }
        }

        for (id, ref mut client) in self.connections.iter_mut() {
            remove_player_on_disconnect!(client.fetch_bytes(), *id);

            let mut player_input_x = 0.0;
            let mut player_input_y = 0.0;
            let mut player_shooting = false;

            // TODO: Use a real loop
            while let Some(message) = client.next() {
                match message {
                    ClientMessage::Input{ x_input, y_input, shooting } => {
                        player_input_x = x_input;
                        player_input_y = y_input;
                        player_shooting = shooting
                    },
                    ClientMessage::JoinGame{ name, plane, color } => {
                        let mut random = rand::thread_rng();

                        let player = Player::new(
                            *id,
                            Point2::new(
                                random.gen_range(0., constants::WORLD_SIZE),
                                random.gen_range(0., constants::WORLD_SIZE)
                                ),
                            plane,
                            color,
                            name
                        );
                        self.state.add_player(player);
                    }
                }
            }

            let mut bullet = None;
            for player in &mut self.state.players {
                if player.id == *id {
                    player.update(
                        player_input_x,
                        player_input_y,
                        delta_time,
                    );

                    if player_shooting {
                        bullet = player.shoot();
                    }

                    let result = send_server_message(
                        &ServerMessage::GameState(self.state.clone()),
                        &mut client.stream
                    );
                    remove_player_on_disconnect!(result, *id);
                    break
                }
            }

            if let Some(bullet) = bullet {
                let pos = bullet.position;
                self.state.add_bullet(bullet);
                sounds_to_play.push((SoundEffect::Gun, pos));
            }
        }

        for (sound, pos) in &sounds_to_play {
            for (id, ref mut client) in self.connections.iter_mut() {
                let result = send_server_message(
                    &ServerMessage::PlaySound(*sound, *pos),
                    &mut client.stream
                );
                remove_player_on_disconnect!(result, *id);
            }
        }

        let dead_players: Vec<_> = self.state.players.iter()
            .filter(|player| player.health <= 0).map(|player| player.id)
            .collect();

        self.state.players.retain(
            |player| !clients_to_delete.contains(&player.id) &&
                !dead_players.contains(&player.id)
        );
        self.connections.retain(
            |(id, _)| !clients_to_delete.contains(id)
        );
    }
}

fn main() {
    let mut server = Server::new();
    loop {
        server.update();
    }
}

