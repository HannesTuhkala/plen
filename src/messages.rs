use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Ping,
}
