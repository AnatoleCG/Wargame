use bevy::prelude::*;

#[derive(Component, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum MovState { Moving, Idle }

#[derive(Component)]
pub struct Position {pub x: f32, pub y: f32,}

#[derive(Component)]
pub struct Angle {pub value: f32,}

#[derive(Component)]
pub struct LinearVelocity {pub x: f32, pub y: f32,}

