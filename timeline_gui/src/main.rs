#![expect(clippy::needless_pass_by_value, reason = "Bevy Queries")]

#[cfg(feature = "debug")]
use bevy::{
    dev_tools::diagnostics_overlay::DiagnosticsOverlayPlugin,
    diagnostic::FrameTimeDiagnosticsPlugin, pbr::diagnostic::MaterialAllocatorDiagnosticPlugin,
    render::diagnostic::MeshAllocatorDiagnosticPlugin,
};
use bevy::{prelude::*, winit::WinitSettings};

use crate::{setup::SetupPlugin, timeline::rendering::TimelineRendererPlugin};

mod dragging;
mod query_ext;
mod setup;
mod timeline;
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
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(ClearColor(Color::hsv(0., 0., 0.3)))
        .add_plugins((
            SetupPlugin,
            TimelineRendererPlugin,
            #[cfg(feature = "debug")]
            (
                FrameTimeDiagnosticsPlugin::default(),
                DiagnosticsOverlayPlugin,
                MaterialAllocatorDiagnosticPlugin::<StandardMaterial>::new(""),
                MeshAllocatorDiagnosticPlugin,
            ),
        ))
        .run()
}
