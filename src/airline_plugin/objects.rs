use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Aircraft {
    pub start_pos: Vec2,
    pub current_pos: Vec2,
    pub end_pos: Vec2,
    pub speed: f32,
    pub remaining_steps: i32,
}

#[derive(Component, Debug)]
pub struct Airport {
    pub pos: Vec2,
    pub name: String,
}

impl Aircraft {
    pub fn new(start_pos: Vec2, end_pos: Vec2, speed: f32) -> Self {
        return Aircraft {
            start_pos,
            end_pos,
            speed,
            current_pos: start_pos,
            remaining_steps: ((start_pos - end_pos).length() / speed) as i32,
        };
    }
}
