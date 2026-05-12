use bevy::{camera::visibility::RenderLayers, prelude::*};
#[cfg(feature = "debug")]
use bevy::{
    dev_tools::diagnostics_overlay::DiagnosticsOverlay,
    input::common_conditions::input_just_pressed,
};
use timeline_core::TimelineManager;

use crate::timeline::{RenderedTimeline, Timeline};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                Self::spawn_timeline,
                Self::spawn_camera,
                #[cfg(feature = "debug")]
                Self::spawn_diagnostics_overlay,
            ),
        );

        #[cfg(feature = "debug")]
        app.add_systems(
            Update,
            Self::toggle_diagnostics_overlay.run_if(input_just_pressed(KeyCode::F1)),
        );
    }
}

impl SetupPlugin {
    fn spawn_camera(mut commands: Commands) {
        commands.spawn((Camera2d, RenderLayers::layer(0), MainCamera));
    }

    fn spawn_timeline(mut commands: Commands) {
        commands.spawn((
            Timeline {
                manager: TimelineManager::new(),
            },
            RenderedTimeline,
            Transform::from_translation(Vec3::splat(0.)),
        ));
    }

    #[cfg(feature = "debug")]
    fn spawn_diagnostics_overlay(mut commands: Commands) {
        commands.spawn(DiagnosticsOverlay::fps());
        commands.spawn((
            DiagnosticsOverlay::mesh_and_standard_material(),
            UiTransform::from_translation(Val2::px(0_u32, 100_u32)),
        ));
    }

    #[cfg(feature = "debug")]
    fn toggle_diagnostics_overlay(diagnostics: Query<&mut Visibility, With<DiagnosticsOverlay>>) {
        for mut visibility in diagnostics {
            visibility.toggle_inherited_hidden();
        }
    }
}

#[derive(Debug, Component)]
pub struct MainCamera;
