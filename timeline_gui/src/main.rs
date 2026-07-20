#![expect(clippy::needless_pass_by_value, reason = "Bevy Queries")]
use bevy::{prelude::*, winit::WinitSettings};

use crate::{setup::SetupPlugin, timeline::rendering::TimelineRendererPlugin};

#[cfg(feature = "debug")]
mod debug;
mod dragging;
mod query_ext;
mod setup;
mod timeline;
mod wrap_around;
mod zooming;

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
            debug::DebugPlugin,
        ))
        .run()
}
