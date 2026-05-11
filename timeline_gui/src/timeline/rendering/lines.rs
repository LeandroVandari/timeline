use bevy::{prelude::*, window::PrimaryWindow};
use timeline_core::date_iteration::YearIterator;
use tracing::instrument;

/// Spawn the lines for each year and corresponding labels for drawing the timelines.
#[instrument(skip_all)]
pub fn spawn_timeline_lines(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    render_info_query: Query<(&super::TimelineRenderInformation, &Transform)>,
    mut added_render_infos: MessageReader<super::TimelineRenderInformationCreatedMessage>,
) {
    for msg in added_render_infos.read() {
        let entity = msg.entity();
        let (render_info, pos) = render_info_query
            .get(entity)
            .expect("Message should refer to an entity with proper components.");
        trace!("Spawning lines for timeline {entity}");
        let timeline_size = render_info.size.unwrap_or(window.size());

        // Main, horizontal line
        commands.entity(entity).with_child((
            Mesh2d(meshes.add(Rectangle::new(timeline_size.x, 3.))),
            MeshMaterial2d(materials.add(Color::srgb(0.9, 0.9, 0.9))),
            pos.with_translation(Vec3::ZERO),
            render_info.layers.clone(),
        ));

        // Vertical lines for years
        let year_line_mesh = meshes.add(Rectangle::new(1., timeline_size.y));
        let year_line_material = materials.add(Color::srgb(0.8, 0.8, 0.8));

        let mut num_lines = (timeline_size.x / render_info.line_dist).ceil() as u32;
        num_lines += 2 - num_lines % 2;
        let mut year_iterator = YearIterator::new(&render_info.year_start).unwrap();
        for i in 0..num_lines {
            let year_x_pos = -(num_lines as f32 * render_info.line_dist)
                + render_info.line_dist * i as f32
                + render_info.horizontal_offset
                + timeline_size.x / 2.;

            commands.entity(entity).with_child((
                Mesh2d(year_line_mesh.clone()),
                MeshMaterial2d(year_line_material.clone()),
                pos.with_translation(Vec3::new(year_x_pos, 0., 0.)),
                render_info.layers.clone(),
                children![(
                    Text2d::new(year_iterator.next().unwrap().to_string()),
                    Transform::from_xyz(0., -15., 0.),
                    render_info.layers.clone()
                )],
            ));
        }
    }
}
