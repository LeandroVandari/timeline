use bevy::prelude::*;
use bevy_timeline::{RenderedTimeline, TimelineRendererPlugin};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Timeline Example - Big Timeline".into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(Color::hsv(0., 0., 0.3)))
        .add_plugins((
            TimelineRendererPlugin,
            #[cfg(feature = "debug")]
            bevy_timeline::debug::DebugPlugin,
        ))
        .add_systems(Startup, spawn_timeline)
        .run()
}

fn spawn_timeline(mut commands: Commands) {
    commands.spawn((
        RenderedTimeline::default(),
        Transform::from_translation(Vec3::splat(0.)),
    ));
}
