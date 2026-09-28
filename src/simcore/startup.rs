use bevy::prelude::*;


pub struct StartupPlugin;
impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}