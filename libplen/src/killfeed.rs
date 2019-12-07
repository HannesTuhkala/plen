use serde_derive::{Serialize, Deserialize};

use crate::constants;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Message {
    pub message: String,
    pub duration_left: Option<f32>,
}

impl Message {
    pub fn new(message: String) -> Self {
        Message {
            message,
            duration_left: Some(constants::KILLFEED_DURATION),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct KillFeed {
    pub messages: Vec<Message>,
}

impl KillFeed {
    pub fn new() -> KillFeed {
        KillFeed {
            messages: vec!(),
        }
    }

    pub fn manage_killfeed(&mut self, delta: f32) {
        // Decrease the messages time left
        self.messages.iter_mut()
            .for_each(|m| {
                // Decrease the time left
                m.duration_left = m.duration_left
                    .map(|left| left - delta)
            });

        // Remove if the time is up
        self.messages
            .retain(|m| {
                m.duration_left
                    .map(|left| left > 0.)
                    .unwrap_or(true)
            });
    }

    pub fn add_message(&mut self, message: &str) {
        self.messages.push(Message::new(message.to_string()));
    }

    pub fn get_messages(&mut self) -> Vec<Message> {
        if self.messages.len() >= 5 {
            self.messages[..4].to_vec()
        } else {
            self.messages[..].to_vec()
        }
    }
}
