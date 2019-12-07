use std::io::{self, prelude::*};
use std::net::TcpStream;
use std::collections::VecDeque;
use std::iter::Iterator;
use std::marker::PhantomData;

use serde_derive::{Serialize, Deserialize};
use nalgebra as na;

use crate::player;

pub struct MessageReader<T> {
    pub stream: TcpStream,
    byte_queue: VecDeque<u8>,
    _0: PhantomData<T>,
}

impl<T> MessageReader<T> {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            byte_queue: VecDeque::new(),
            _0: PhantomData
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
}

macro_rules! impl_message_reader {
    ($output:ty) => {
        impl Iterator for MessageReader<$output>
        {
            type Item = $output;

            fn next(&mut self) -> Option<Self::Item> {
                // We need two bytes for the length
                if self.byte_queue.len() < 2 {
                    return None;
                }

                let length = u16::from_be_bytes([
                    self.byte_queue[0],
                    self.byte_queue[1],
                ]) as usize;

                // We will not read a message until a complete message has been
                // received
                if self.byte_queue.len() < 2 + length {
                    return None;
                }

                self.byte_queue.pop_front().unwrap();
                self.byte_queue.pop_front().unwrap();

                let msg_bytes: Vec<_> = self.byte_queue.drain(0..length).collect();

                Some(
                    bincode::deserialize(&msg_bytes)
                        .expect("Failed to decode message")
                )
            }
        }
    }
}

impl_message_reader!(ServerMessage);
impl_message_reader!(ClientMessage);

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum SoundEffect { Powerup, Explosion, Gun, LaserCharge, LaserFire }

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    AssignId(u64),
    GameState(crate::gamestate::GameState),
    PlaySound(SoundEffect, na::Point2<f32>),
    PlayerHit(u64),
    YouDied,
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    Input { x_input: f32, y_input: f32, shooting: bool, firing_powerup: bool },
    JoinGame { name: String, plane: player::PlaneType, color: player::Color },
}
