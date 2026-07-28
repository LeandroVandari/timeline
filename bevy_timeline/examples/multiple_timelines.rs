use bevy::{prelude::*, winit::WinitSettings};
use bevy_timeline::{
    RenderedTimeline, TimelineRendererPlugin,
    configuration::{TimelineLineSeparation, TimelineRenderRange, TimelineScreenSize},
};
use temporal_rs::PlainDate;
use timeline_core::date_iteration::{YearRange, year::Year};

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
        Transform::from_translation(Vec3::new(200., -100., 0.)),
        TimelineScreenSize(Vec2 { x: 500., y: 100. }),
        TimelineLineSeparation(40.),
        TimelineRenderRange(YearRange {
            start: Year::from(PlainDate::new_iso(1, 1, 1).unwrap()),
            end: Year::from(PlainDate::new_iso(21, 1, 1).unwrap()),
        }),
    ));

    /*commands.spawn((
        RenderedTimeline,
        TimelineScreenSize(Vec2 { x: 200., y: 103. }),
        TimelineLineSeparation(88.),
        Transform::from_translation(Vec3::splat(-100.))
            .with_rotation(Quat::from_rotation_z(core::f32::consts::PI / 4.)),
    )); */

    commands.spawn((
        RenderedTimeline::default(),
        TimelineScreenSize(Vec2 { x: 200., y: 100. }),
        Transform::from_translation(Vec3::new(200., -300., 0.)),
        TimelineRenderRange(YearRange {
            start: Year::from(PlainDate::new_iso(-20, 1, 1).unwrap()),
            end: Year::from(PlainDate::new_iso(20, 1, 1).unwrap()),
        }),
    ));

    commands.spawn((
        RenderedTimeline::default(),
        TimelineScreenSize(Vec2 { x: 500., y: 300. }),
        Transform::from_translation(Vec3::new(-300., 200., 0.)),
    ));
}
