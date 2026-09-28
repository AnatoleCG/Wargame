use bevy::prelude::*;
use crate::components::transforms::*;

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement_system);
    }
    fn movement_system(
        mut query: Query<(&mut Position, &LinearVelocity, &MovState)>,
        time: Res<Time>,
    ) {
        for (mut position, velocity, state) in query.iter_mut() {
            position.x += velocity.x * time.delta_seconds();
            position.y += velocity.y * time.delta_seconds();
        }
    }
}