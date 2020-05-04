use std::io::{self, prelude::*};
use std::net::TcpStream;
use std::collections::VecDeque;
use std::iter::Iterator;

use serde_derive::{Serialize, Deserialize};

use crate::player;
use crate::math::Vec2;

pub struct MessageReader {
    pub stream: TcpStream,
    byte_queue: VecDeque<u8>,
}

pub struct MessageIterator<'a> {
    message_reader: &'a mut MessageReader
}

impl MessageReader {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            byte_queue: VecDeque::new(),
        }
    }

    pub fn fetch_bytes(&mut self) -> io::Result<()> {
        let mut buffer = [1; 64];
        loop {
            let amount = match self.stream.read(&mut buffer) {
                Ok(amount) => amount,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => 0,
                e => e?,
            };
            if amount == 0 {
                break Ok(());
            }
            self.byte_queue.extend(buffer.iter().take(amount));
        }
    }

    pub fn iter<'a>(&'a mut self) -> MessageIterator<'a> {
        MessageIterator {
            message_reader: self
        }
    }
}

impl Iterator for MessageIterator<'_> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        // We need two bytes for the length
        if self.message_reader.byte_queue.len() < 2 {
            return None;
        }

        let length = u16::from_be_bytes([
            self.message_reader.byte_queue[0],
            self.message_reader.byte_queue[1],
        ]) as usize;

        // We will not read a message until a complete message has been
        // received
        if self.message_reader.byte_queue.len() < 2 + length {
            return None;
        }

        self.message_reader.byte_queue.pop_front().unwrap();
        self.message_reader.byte_queue.pop_front().unwrap();

        Some(self.message_reader.byte_queue.drain(0..length).collect())
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum SoundEffect { Powerup, Explosion, Gun, LaserCharge, LaserFire }

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    AssignId(u64),
    GameState(crate::gamestate::GameState),
    PlaySound(SoundEffect, Vec2),
    PlayerHit(u64),
    YouDied,
}

#[derive(Serialize, Deserialize)]
pub struct ClientInput {
    pub x_input: f32,
    pub y_input: f32,
    pub shooting: bool,
    pub activating_powerup: bool,
}

impl ClientInput {
    pub fn new() -> Self {
        ClientInput {
            x_input: 0.,
            y_input: 0.,
            shooting: false,
            activating_powerup: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    Input(ClientInput),
    JoinGame { name: String, plane: player::PlaneType, color: player::Color },
}
