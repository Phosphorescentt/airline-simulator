use bevy::prelude::*;

#[derive(Component)]
pub struct Aircraft {
    pub start_pos: Vec2,
    pub current_pos: Vec2,
    pub end_pos: Vec2,
    pub speed: f32,
}

#[derive(Component)]
pub struct Airport {
    pub pos: Vec2,
    pub name: String,
}
