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
                present_mode: if cfg!(feature = "profiling") {
                    bevy::window::PresentMode::AutoNoVsync
                } else {
                    bevy::window::PresentMode::Fifo
                },
                #[cfg(feature = "wasm_website")]
                canvas: Some("#timeline-canvas".into()),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
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
