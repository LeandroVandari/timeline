use bevy::prelude::*;
use bevy_timeline::{
    RenderedTimeline, TimelineRendererPlugin,
    configuration::{TimelineRenderRange, TimelineScreenSize},
};
use temporal_rs::PlainDate;
use timeline_core::date_iteration::{YearRange, year::Year};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Timeline Example - Small Timeline".into(),
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
        TimelineScreenSize(Vec2 { x: 200., y: 100. }),
        TimelineRenderRange(YearRange {
            start: Year::from(PlainDate::new_iso(-20, 1, 1).unwrap()),
            end: Year::from(PlainDate::new_iso(20, 1, 1).unwrap()),
        }),
    ));
}
