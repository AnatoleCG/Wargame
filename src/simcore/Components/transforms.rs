use bevy::prelude::*;

#[derive(Component)]
pub enum MovState { Moving, Idle, running,}
impl MovState {
    pub fn new() -> Self {
        MovState::Idle
    }
}

#[derive(Component)]
pub struct Position {pub x: f32, pub y: f32, pub heading: f32,}
impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position { x, y, heading: 0.0 }
    }
}

#[derive(Component)]
pub struct LinearVelocity {pub vx: f32, pub vy: f32,}
impl LinearVelocity {
    pub fn new(vx: f32, vy: f32) -> Self {
        LinearVelocity { vx, vy }
    }
}
