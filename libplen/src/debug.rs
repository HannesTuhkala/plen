use serde_derive::{Serialize, Deserialize};

use std::sync::mpsc;


use nalgebra as na;

#[derive(Serialize, Deserialize, Clone)]
pub struct DebugLine {
    pub start: na::Point2<f32>,
    pub end: na::Point2<f32>,
    pub color: (u8, u8, u8, u8),
}


impl DebugLine {
    pub fn new(start: na::Point2<f32>, end: na::Point2<f32>) -> Self {
        Self {start, end, color: (255, 255, 255, 255)}
    }

    pub fn from_angle(start: na::Point2<f32>, angle: f32, length: f32) -> Self {
        Self {
            start,
            end: start + na::Vector2::new(
                angle.cos() * length,
                angle.sin() * length,
            ),
            color: (255, 255, 255, 255)
        }
    }

    pub fn rgb(self, color: (u8, u8, u8)) -> Self {
        self.rgba((color.0, color.1, color.2, 255))
    }
    pub fn rgba(self, color: (u8, u8, u8, u8)) -> Self {
        Self {color, .. self}
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
