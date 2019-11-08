use std::vec;
use serde_derive::{Serialize, Deserialize};

use crate::player::Player;
use crate::bullet::Bullet;

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub players: Vec<Player>,
    pub bullets: Vec<Bullet>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            players: Vec::new(),
            bullets: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player)
    }
}
