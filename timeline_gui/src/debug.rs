use bevy::{
    camera::visibility::RenderLayers,
    dev_tools::diagnostics_overlay::{DiagnosticsOverlay, DiagnosticsOverlayPlugin},
    diagnostic::FrameTimeDiagnosticsPlugin,
    input::common_conditions::input_just_pressed,
    pbr::diagnostic::MaterialAllocatorDiagnosticPlugin,
    prelude::*,
    render::diagnostic::MeshAllocatorDiagnosticPlugin,
    window::PrimaryWindow,
};

use crate::{
    timeline::rendering::{
        configuration::{
            TimelineHorizontalOffset, TimelineLineSeparation, TimelineRenderRange,
            TimelineScreenSize,
        },
        draw_width,
    },
    zooming::ZoomLevel,
};

#[derive(Default)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin::default(),
            DiagnosticsOverlayPlugin,
            MaterialAllocatorDiagnosticPlugin::<StandardMaterial>::new(""),
            MeshAllocatorDiagnosticPlugin,
        ));

        app.add_systems(
            PreStartup,
            (Self::spawn_diagnostics_overlay, Self::spawn_full_view_cam),
        );
        app.add_systems(
            Update,
            (
                Self::draw_timeline_gizmos,
                Self::toggle_diagnostics_overlay.run_if(input_just_pressed(KeyCode::F1)),
                Self::toggle_full_view_cam.run_if(input_just_pressed(KeyCode::F2)),
                Self::update_full_view_cam_render_layers,
            ),
        );
    }
}

impl DebugPlugin {
    #[expect(clippy::type_complexity, reason = "Bevy Queries")]
    pub fn draw_timeline_gizmos(
        mut gizmos: Gizmos,
        query: Query<(
            &TimelineHorizontalOffset,
            &ZoomLevel,
            &TimelineLineSeparation,
            &TimelineRenderRange,
            &Transform,
            Option<&TimelineScreenSize>,
        )>,
        window: Single<&Window, With<PrimaryWindow>>,
    ) {
        for (hoffset, &zoom, &line_separation, render_range, pos, screen_size) in query.iter() {
            let size = screen_size.map_or(window.size(), |s| **s);
            gizmos.line_2d(
                Vec2::new(
                    pos.translation.x + **hoffset,
                    pos.translation.y - size.y.midpoint(100.),
                ),
                Vec2::new(
                    pos.translation.x + **hoffset,
                    pos.translation.y + size.y.midpoint(100.),
                ),
                Color::linear_rgb(0., 0., 1.),
            );
            gizmos.rect_2d(
                Isometry2d::from_translation(Vec2::new(
                    pos.translation.x + **hoffset,
                    pos.translation.y,
                )),
                Vec2::new(draw_width(render_range, line_separation, zoom), size.y),
                Color::linear_rgb(0., 1., 0.),
            );
        }
    }

    fn spawn_diagnostics_overlay(mut commands: Commands) {
        commands.spawn((DiagnosticsOverlay::fps(), Visibility::Hidden));
        commands.spawn((
            DiagnosticsOverlay::mesh_and_standard_material(),
            UiTransform::from_translation(Val2::px(0_u32, 100_u32)),
            Visibility::Hidden,
        ));
    }

    fn toggle_diagnostics_overlay(diagnostics: Query<&mut Visibility, With<DiagnosticsOverlay>>) {
        for mut visibility in diagnostics {
            visibility.toggle_inherited_hidden();
        }
    }

    fn toggle_full_view_cam(mut cam: Single<&mut Camera, With<FullViewCamera>>) {
        cam.is_active = !cam.is_active;
    }

    fn spawn_full_view_cam(mut commands: Commands) {
        commands.spawn((
            Camera {
                order: 100,
                is_active: false,
                ..Default::default()
            },
            Camera2d,
            FullViewCamera,
            RenderLayers::none(),
        ));
    }

    fn update_full_view_cam_render_layers(
        mut cam_query: Single<&mut RenderLayers, With<FullViewCamera>>,
        render_layers_query: Query<&RenderLayers, (Without<FullViewCamera>, Changed<RenderLayers>)>,
    ) {
        for added_layers in render_layers_query {
            **cam_query = (*cam_query).union(added_layers);
        }
    }
}

#[derive(Component, Debug)]
struct FullViewCamera;
