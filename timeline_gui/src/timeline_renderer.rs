use bevy::log::tracing::instrument;
use bevy::{prelude::*, window::PrimaryWindow};
use timeline_core::date_iteration::YearIterator;

use crate::timeline::{Timeline, render_information::TimelineRenderInformation};

pub struct TimelineRendererPlugin;

impl Plugin for TimelineRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::spawn_timeline_render_components);
    }
}

impl TimelineRendererPlugin {
    #[instrument(skip_all)]
    fn spawn_timeline_render_components(
        mut commands: Commands,
        added_render_info: Query<(Entity, &TimelineRenderInformation, &Transform), Added<Timeline>>,
        window: Single<&Window, With<PrimaryWindow>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let window_size = window.size();

        for (entity, render_info, pos) in added_render_info {
            info!(
                "Spawning timeline for entity {} with render configuration {:#?}",
                entity, render_info
            );
            let timeline_size = render_info.size.unwrap_or(window_size);

            // Main, horizontal line
            let main_line = commands
                .spawn((
                    Mesh2d(meshes.add(Rectangle::new(timeline_size.x, 3.))),
                    MeshMaterial2d(materials.add(Color::srgb(0.9, 0.9, 0.9))),
                    *pos,
                ))
                .id();
            commands.entity(entity).add_child(main_line);

            // Vertical lines for years
            let year_line_mesh = meshes.add(Rectangle::new(1., timeline_size.y));
            let year_line_material = materials.add(Color::srgb(0.8, 0.8, 0.8));

            let num_lines = (timeline_size.x / render_info.line_dist).floor() as u32;
            let mut year_iterator = YearIterator::new(&render_info.year_start).unwrap();
            for i in 0..num_lines {
                let year_x_pos = pos.translation.x
                    + render_info.line_dist * i as f32
                    + render_info.horizontal_offset
                    - timeline_size.x / 2.;
                let vertical_line = commands
                    .spawn((
                        Mesh2d(year_line_mesh.clone()),
                        MeshMaterial2d(year_line_material.clone()),
                        Transform::from_xyz(year_x_pos, pos.translation.y, 0.),
                    ))
                    .with_child((
                        Text2d::new(year_iterator.next().unwrap().to_string()),
                        // Transform from child is just parent offset
                        Transform::from_xyz(0., -15., 0.),
                    ))
                    .id();
                commands.entity(entity).add_child(vertical_line);
            }
        }
    }
}
