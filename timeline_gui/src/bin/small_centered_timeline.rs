use bevy::{prelude::*, winit::WinitSettings};
use temporal_rs::PlainDate;
use timeline_core::date_iteration::{YearRange, year::Year};
use timeline_gui::{
    setup::SetupPlugin,
    timeline::{
        RenderedTimeline,
        rendering::{
            TimelineRendererPlugin,
            configuration::{TimelineRenderRange, TimelineScreenSize},
        },
    },
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Test Timelines".into(),
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
        TimelineScreenSize(Vec2 { x: 200., y: 100. }),
        TimelineRenderRange(YearRange {
            start: Year::from(PlainDate::new_iso(-20, 1, 1).unwrap()),
            end: Year::from(PlainDate::new_iso(20, 1, 1).unwrap()),
        }),
    ));
}
