use macroquad::math::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir4 {
    Up,
    Right,
    Down,
    Left,
}

pub fn dir_from_vec2(v: Vec2) -> Option<Dir4> {
    if v.x.abs() > v.y.abs() {
        if v.x > 0.0 {
            Some(Dir4::Right)
        } else if v.x < 0.0 {
            Some(Dir4::Left)
        } else {
            None
        }
    } else {
        if v.y > 0.0 {
            Some(Dir4::Down)
        } else if v.y < 0.0 {
            Some(Dir4::Up)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Horizontal2 {
    Left,
    Right,
}
