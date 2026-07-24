use bevy::{prelude::*, winit::WinitSettings};
use timeline_gui::{
    setup::SetupPlugin,
    timeline::{RenderedTimeline, rendering::TimelineRendererPlugin},
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Timeline Creator".into(),
                present_mode: bevy::window::PresentMode::Fifo,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(if cfg!(feature = "profiling") {
            WinitSettings::continuous()
        } else {
            WinitSettings::desktop_app()
        })
        .insert_resource(ClearColor(Color::hsv(0., 0., 0.3)))
        .add_plugins((
            SetupPlugin,
            TimelineRendererPlugin,
            #[cfg(feature = "debug")]
            timeline_gui::debug::DebugPlugin,
        ))
        .add_systems(Startup, spawn_timeline)
        .run()
}

fn spawn_timeline(mut commands: Commands) {
    commands.spawn((
        RenderedTimeline,
        Transform::from_translation(Vec3::splat(0.)),
    ));
}
