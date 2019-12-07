use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum GunType {
	Regular,
	Laser,
	Minigun, 
}

