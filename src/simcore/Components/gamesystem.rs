use bevy::prelude::*;

enum GameStateEnum {
    Idle,
    Playing,
    GameOver,
}

#[derive(Resource)]
pub struct EntityCounter {pub count: u32,}

#[derive(Resource)]
pub struct GameState {pub state: GameStateEnum,}

#[derive(Resource)]
pub struct KillCounter {pub count: Vec<u32>,}