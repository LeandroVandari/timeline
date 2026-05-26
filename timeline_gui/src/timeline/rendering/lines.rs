use bevy::{camera::visibility::RenderLayers, prelude::*, window::PrimaryWindow};
use timeline_core::date_iteration::year::Year;
use tracing::instrument;

use crate::timeline::rendering::{
    configuration::{TimelineLineSeparation, TimelineRenderRange, TimelineScreenSize},
    dragging::{
        HorizontalWrapAround, WrapAround, WrapDirection,
        relationship::{DraggedBy, HorizontallyDraggedBy, VerticallyDraggedBy},
    },
};

/// Spawn the lines for each year and corresponding labels for drawing the timelines.
#[instrument(skip_all)]
pub fn spawn_timeline_lines(
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
    )>,
    mut added_render_infos: MessageReader<super::RenderedTimelineCreatedMessage>,
) {
    for msg in added_render_infos.read() {
        let entity = msg.entity();
        let (size, pos, render_layers, &line_separation, render_range) = render_info_query
            .get(entity)
            .expect("Message should refer to an entity with proper components.");
        trace!("Spawning lines for timeline {entity}");
        let render_size = size.map_or(window.size(), |s| **s);

        // Main, horizontal line
        commands.entity(entity).with_child((
            VerticallyDraggedBy(entity),
            Mesh2d(meshes.add(Rectangle::new(render_size.x, 3.))),
            MeshMaterial2d(materials.add(Color::srgb(0.9, 0.9, 0.9))),
            pos.with_translation(Vec3::ZERO),
            render_layers.clone(),
        ));

        // Vertical lines for years
        let year_line_mesh = meshes.add(Rectangle::new(1., render_size.y));
        let year_line_material = materials.add(Color::srgb(0.8, 0.8, 0.8));

        let year_iterator = render_range.0.into_iter();
        let draw_width = (render_range.0.len() + 1) as f32 * *line_separation;
        for (i, year) in year_iterator.enumerate() {
            let year_x_pos = -draw_width / 2. + *line_separation * i as f32;

            commands.entity(entity).with_children(|spawner| {
                spawner.spawn((
                    HorizontalWrapAround {
                        center: pos.translation.x,
                        half_width: draw_width / 2.,
                        emit_message: false,
                    },
                    HorizontallyDraggedBy(entity),
                    Mesh2d(year_line_mesh.clone()),
                    MeshMaterial2d(year_line_material.clone()),
                    pos.with_translation(Vec3::new(year_x_pos, 0., 0.)),
                    render_layers.clone(),
                ));
                spawner
                    .spawn((
                        HorizontalWrapAround {
                            center: pos.translation.x,
                            half_width: draw_width / 2.,
                            emit_message: true,
                        },
                        DraggedBy::new(entity),
                        Text2d::new(year.to_string()),
                        YearLabel(year),
                        pos.with_translation(Vec3::new(year_x_pos, -15., 0.)),
                        render_layers.clone(),
                    ))
                    .observe(
                        move |trigger: On<WrapAround>,
                              mut label_query: Query<(&mut YearLabel, &mut Text2d)>,
                              mut range_query: Query<&mut TimelineRenderRange>| {
                            let (mut year, mut text_label) =
                                label_query.get_mut(trigger.entity).unwrap();
                            let mut range = range_query.get_mut(entity).unwrap();
                            match trigger.direction {
                                WrapDirection::Left => {
                                    range.inc();
                                    year.0 = range.0.end.clone()},
                                WrapDirection::Right => {
                                    range.dec();
                                    year.0 = range.0.start.clone()
                                }
                            };
                            text_label.0 = year.0.to_string()
                        },
                    );
            });
        }
    }
}

#[derive(Debug, Component)]
struct YearLabel(pub Year);
