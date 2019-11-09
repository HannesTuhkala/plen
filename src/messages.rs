use std::io::{self, prelude::*};
use serde_derive::{Serialize, Deserialize};
use std::net::TcpStream;
use std::collections::VecDeque;
use std::iter::Iterator;
use std::marker::PhantomData;

pub struct MessageReader<T> {
    stream: TcpStream,
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
                let mut first_null = None;
                for (i, b) in self.byte_queue.iter().enumerate() {
                    if *b == '\0' as u8 {
                        first_null = Some(i);
                        break;
                    }
                }

                let msg_bytes = match first_null {
                    None => return None,
                    Some(i) => self.byte_queue.drain(0..i)
                }.collect::<Vec<u8>>();
                self.byte_queue.pop_front();

                let as_str = String::from_utf8_lossy(&msg_bytes);

                Some(
                    serde_json::from_str(&as_str).expect("Failed to decode message")
                )
            }
        }
    }
}

impl_message_reader!(ServerMessage);
impl_message_reader!(ClientMessage);



#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    Ping,
    AssignId(u64),
    GameState(crate::gamestate::GameState)
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    Ping,
    Shoot,
}
