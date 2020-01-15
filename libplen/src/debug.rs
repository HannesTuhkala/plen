use std::sync::mpsc;
use serde_derive::{Serialize, Deserialize};
use crate::math::Vec2;

#[derive(Serialize, Deserialize, Clone)]
pub struct DebugLine {
    pub start: Vec2,
    pub end: Vec2,
    pub color: (u8, u8, u8, u8),
}


impl DebugLine {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self {start, end, color: (255, 255, 255, 255)}
    }

    pub fn from_angle(start: Vec2, angle: f32, length: f32) -> Self {
        Self {
            start,
            end: start + Vec2::from_direction(angle, length),
            color: (255, 255, 255, 255)
        }
    }

    pub fn rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.rgba(r, g, b,  255)
    }
    pub fn rgba(self, r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {color: (r, g, b, a), .. self}
    }
}


static mut DEBUG_CHANNEL: Option<mpsc::Sender<DebugLine>> = None;

pub fn init_debug_channel() -> mpsc::Receiver<DebugLine> {
    let (rx, tx) = mpsc::channel();

    unsafe {
        DEBUG_CHANNEL.replace(rx);
    }

    tx
}

// NOTE: Kind of unsafe. Will panic if init_debug_channel hasn't been called
// yet
pub fn debug_channel() -> mpsc::Sender<DebugLine> {
    unsafe {
        DEBUG_CHANNEL.clone().take()
            .expect("Trying to get debug channel before initing debug system")
    }
}

pub fn send_line(line: DebugLine) {
    debug_channel().send(line).unwrap();
}
