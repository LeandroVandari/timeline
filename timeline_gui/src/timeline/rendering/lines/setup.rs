use bevy::{camera::visibility::RenderLayers, prelude::*, window::PrimaryWindow};
use tracing::instrument;

use crate::{
    dragging::relationship::VerticallyDraggedBy,
    timeline::rendering::{
        configuration::{TimelineLineSeparation, TimelineRenderRange, TimelineScreenSize},
        draw_width,
    },
    wrap_around::WrapAroundInfo,
    zooming::ZoomLevel,
};

use super::VerticalLineRenderInfo;

impl super::TimelineLinesPlugin {
    #[tracing::instrument(skip_all)]
    pub(super) fn create_vertical_line_render_info(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut added_render_infos: MessageReader<super::RenderedTimelineCreatedMessage>,
        timeline_info_query: Query<(
            &Transform,
            Option<&TimelineScreenSize>,
            &TimelineRenderRange,
            &TimelineLineSeparation,
            &ZoomLevel,
        )>,
        window: Single<&Window, With<PrimaryWindow>>,
    ) {
        for added_render_info in added_render_infos.read() {
            trace!(
                "Creating vertical line render info for timeline {}",
                added_render_info.entity()
            );
            let (timeline_pos, size, render_range, &line_separation, &zoom_level) =
                timeline_info_query.get(added_render_info.entity()).unwrap();
            let render_size = size.map_or(window.size(), |s| **s);

            let draw_width = draw_width(render_range, line_separation, zoom_level);

            commands.entity(added_render_info.entity()).insert((
                VerticalLineRenderInfo {
                    mesh: meshes.add(Rectangle::new(1., render_size.y)),
                    material: materials.add(Color::srgb(0.8, 0.8, 0.8)),
                },
                WrapAroundInfo {
                    center: timeline_pos.translation.x,
                    // Needs a half-line buffer on each size so the line doesn't teleport exacly on top of the one on the other side.
                    // Midpoint: (occupied_space / 2) + (scaled_line_separation / 2)
                    half_width: f32::midpoint(draw_width, *line_separation * *zoom_level),
                    emit_message: true,
                },
            ));
        }
    }

    /// Spawn the lines for each year and corresponding labels for drawing the timelines.
    #[instrument(skip_all)]
    #[expect(
        clippy::cast_precision_loss,
        reason = "Layouting the timeline is best effort, losing some precision is fine and should only happen for huge values"
    )]
    #[expect(clippy::type_complexity, reason = "Bevy's queries are a complex type")]
    pub(super) fn spawn_timeline_lines(
        mut commands: Commands,
        window: Single<&Window, With<PrimaryWindow>>,

        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,

        render_info_query: Query<(
            Option<&TimelineScreenSize>,
            &Transform,
            &RenderLayers,
            &TimelineLineSeparation,
            &TimelineRenderRange,
            &ZoomLevel,
        )>,
        mut added_render_infos: MessageReader<super::RenderedTimelineCreatedMessage>,

        timeline_info: Query<(&Transform, &VerticalLineRenderInfo, &RenderLayers)>,
    ) {
        for msg in added_render_infos.read() {
            let timeline_entity = msg.entity();
            let (size, pos, render_layers, &line_separation, render_range, &zoom_level) =
                render_info_query
                    .get(timeline_entity)
                    .expect("Message should refer to an entity with proper components.");
            trace!("Spawning lines for timeline {timeline_entity}");
            let render_size = size.map_or(window.size(), |s| **s);

            // Main, horizontal line
            commands.entity(timeline_entity).with_child((
                VerticallyDraggedBy(timeline_entity),
                Mesh2d(meshes.add(Rectangle::new(render_size.x, 3.))),
                MeshMaterial2d(materials.add(Color::srgb(0.9, 0.9, 0.9))),
                pos.with_translation(Vec3::ZERO),
                render_layers.clone(),
            ));

            // Vertical lines for years

            let year_iterator = render_range.0.into_iter();
            let draw_width = draw_width(render_range, line_separation, zoom_level);
            Self::spawn_vertical_lines(
                &mut commands,
                timeline_entity,
                timeline_info,
                year_iterator.enumerate().map(move |(i, year)| {
                    (
                        (*line_separation * *zoom_level).mul_add(i as f32, -draw_width / 2.),
                        year,
                    )
                }),
            )
            .expect("The timeline entity has all the required components.");
        }
    }
}
