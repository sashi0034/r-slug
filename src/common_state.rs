use macroquad::prelude::*;

pub struct CommonState {
    pub time: f32,
}

impl CommonState {
    pub fn new() -> CommonState {
        CommonState { time: 0.0 }
    }
}
