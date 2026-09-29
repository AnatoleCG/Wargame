use bevy::prelude::*;
use components::*;
use movement::MovementPlugin;
use startup::StartupPlugin;

fn main() {
    App::new()
        .add_startup_system(setup)
        .insert_resource(EntityCounter { count: 0 })
        .insert_resource(GameState { state: GameStateEnum::Idle })
        .insert_resource(KillCounter { count: Vec::new() })
        .insert_plugin(StartupPlugin)
        .insert_plugin(MovementPlugin)
        .run();
}

fn setup(mut commands: Commands) {
}