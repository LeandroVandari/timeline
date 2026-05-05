use bevy::prelude::*;
use timeline_core::TimelineManager;

use crate::timeline::{Timeline, render_information::TimelineRenderInformation};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (Self::spawn_timeline, Self::spawn_camera));
    }
}

impl SetupPlugin {
    fn spawn_camera(mut commands: Commands) {
        commands.spawn(Camera2d);
    }

    fn spawn_timeline(mut commands: Commands) {
        commands.spawn((
            Timeline {
                manager: TimelineManager::new(),
            },
            TimelineRenderInformation::default(),
            Transform::from_translation(Vec3::splat(0.)),
        ));
    }
}
