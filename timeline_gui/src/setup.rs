use bevy::prelude::*;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::spawn_camera);
    }
}

impl SetupPlugin {
    fn spawn_camera(mut commands: Commands) {
        commands.spawn((Camera2d, MainCamera));
    }
}

#[derive(Debug, Component)]
pub struct MainCamera;
