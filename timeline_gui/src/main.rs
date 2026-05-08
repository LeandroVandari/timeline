use bevy::prelude::*;

use crate::{setup::SetupPlugin, timeline::rendering::TimelineRendererPlugin};

mod setup;
mod timeline;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Timeline Creator".into(),
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(Color::hsv(0., 0., 0.3)))
        .add_plugins((SetupPlugin, TimelineRendererPlugin))
        .run()
}
