use bevy::{camera::visibility::RenderLayers, prelude::*, window::PrimaryWindow};
use timeline_core::date_iteration::{YearIterator, year::Year};
use tracing::instrument;

use crate::timeline::rendering::{
    configuration::{
        TimelineHorizontalRenderMargin, TimelineLineSeparation, TimelineSize, TimelineStartYear,
    },
    dragging::{
        HorizontalWrapAround, WrapAround, WrapDirection,
        relationship::{DraggedBy, HorizontallyDraggedBy, VerticallyDraggedBy},
    },
};

/// Spawn the lines for each year and corresponding labels for drawing the timelines.
#[expect(
    clippy::type_complexity,
    reason = "Timeline rendering info components are separated and thus a 'complex' type."
)]
#[instrument(skip_all)]
pub fn spawn_timeline_lines(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    render_info_query: Query<(
        Option<&TimelineSize>,
        &Transform,
        &RenderLayers,
        &TimelineLineSeparation,
        &TimelineStartYear,
        &TimelineHorizontalRenderMargin,
    )>,
    mut added_render_infos: MessageReader<super::RenderedTimelineCreatedMessage>,
) {
    for msg in added_render_infos.read() {
        let entity = msg.entity();
        let (size, pos, render_layers, &line_separation, start_year, &horizontal_render_margin) =
            render_info_query
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

        let requested_draw_width = render_size.x * (1. + *horizontal_render_margin);
        let num_lines = (requested_draw_width / *line_separation).ceil();
        let draw_width = num_lines * *line_separation;
        let num_lines = num_lines as u32;

        let mut year_iterator = YearIterator::new(start_year).unwrap();
        for i in 0..num_lines {
            let year_x_pos = -draw_width / 2. + *line_separation * i as f32;

            let year = year_iterator.next().unwrap();
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
                spawner.spawn((
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
                )).observe(|trigger: On<WrapAround>, mut label_query: Query<(&mut YearLabel, &mut Text2d)>| {
                    let (mut year, mut text_label) = label_query.get_mut(trigger.entity).unwrap();
                    match trigger.direction {
                        WrapDirection::Left => year.0 = year.0.get_next().unwrap(),
                        WrapDirection::Right => year.0 = year.0.get_previous().unwrap()
                    };
                    text_label.0 = year.0.to_string()
                });
            });
        }
    }
}

#[derive(Debug, Component)]
struct YearLabel(pub Year);
