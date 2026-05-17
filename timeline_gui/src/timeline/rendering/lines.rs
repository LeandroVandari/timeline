use bevy::{camera::visibility::RenderLayers, prelude::*, window::PrimaryWindow};
use timeline_core::date_iteration::YearIterator;
use tracing::instrument;

use crate::timeline::rendering::{
    configuration::{
        TimelineHorizontalOffset, TimelineLineSeparation, TimelineSize, TimelineStartYear,
    },
    dragging::relationship::DraggedBy,
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
        &TimelineHorizontalOffset,
    )>,
    mut added_render_infos: MessageReader<super::RenderedTimelineCreatedMessage>,
) {
    for msg in added_render_infos.read() {
        let entity = msg.entity();
        let (size, pos, render_layers, &line_separation, start_year, &horizontal_offset) =
            render_info_query
                .get(entity)
                .expect("Message should refer to an entity with proper components.");
        trace!("Spawning lines for timeline {entity}");
        let render_size = size.map_or(window.size(), |s| **s);

        // Main, horizontal line
        commands.entity(entity).with_child((
            MainLine,
            Mesh2d(meshes.add(Rectangle::new(render_size.x, 3.))),
            MeshMaterial2d(materials.add(Color::srgb(0.9, 0.9, 0.9))),
            pos.with_translation(Vec3::ZERO),
            render_layers.clone(),
        ));

        // Vertical lines for years
        let year_line_mesh = meshes.add(Rectangle::new(1., render_size.y));
        let year_line_material = materials.add(Color::srgb(0.8, 0.8, 0.8));

        let mut num_lines = (render_size.x / *line_separation).ceil() as u32;
        num_lines += 2 - num_lines % 2;
        let mut year_iterator = YearIterator::new(start_year).unwrap();
        for i in 0..num_lines {
            let year_x_pos = -(num_lines as f32 * *line_separation)
                + *line_separation * i as f32
                + *horizontal_offset
                + render_size.x / 2.;

            commands.entity(entity).with_children(|spawner| {
                spawner.spawn((
                    VerticalLine,
                    Mesh2d(year_line_mesh.clone()),
                    MeshMaterial2d(year_line_material.clone()),
                    pos.with_translation(Vec3::new(year_x_pos, 0., 0.)),
                    render_layers.clone(),
                ));
                spawner.spawn((
                    DraggedBy(entity),
                    Text2d::new(year_iterator.next().unwrap().to_string()),
                    pos.with_translation(Vec3::new(year_x_pos, -15., 0.)),
                    render_layers.clone(),
                ));
            });
        }
    }
}

#[derive(Debug, Component)]
pub struct VerticalLine;
#[derive(Debug, Component)]
pub struct MainLine;
