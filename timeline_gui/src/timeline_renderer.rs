use bevy::camera::Viewport;
use bevy::camera::visibility::RenderLayers;
use bevy::log::tracing::instrument;
use bevy::{prelude::*, window::PrimaryWindow};
use timeline_core::date_iteration::YearIterator;

use crate::setup::MainCamera;
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
        main_camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    ) {
        let window_size = window.size();

        // Note here that for all entities spawned with a Transform, that transform is *relative* to the parent's transform. This is very important for all
        // position calculations: since we create a custom camera at a world position, Transform::ZERO is the *center of the new camera*, rather than the center of the screen.
        for (entity, render_info, pos) in added_render_info {
            info!(
                "Spawning timeline for entity {} with render configuration {:#?}",
                entity, render_info
            );
            let mut entity_commands = commands.entity(entity);
            let timeline_size = render_info.size.unwrap_or(window_size);

            // Make sure only the proper area from the timeline is drawn by creating a custom camera just to render it whose viewport spans exactly
            // the size of the timeline.
            // This is needed so that events/labels can get cut off rather than popping into existance.
            // I'm not sure of the performance implications of creating two cameras, but I'm pretty sure it's not too bad.
            let layer = RenderLayers::layer(entity.index_u32() as usize);

            // The viewport position is on the top-left corner. In order to convert the pos translation (which has (0,0) at the center) to that,
            // we need to move the coords left and up, which due to Bevy's coordinate system means adding y and subtracting x.
            let viewport_pos = pos
                .translation
                .with_x(pos.translation.x - timeline_size.x / 2.)
                .with_y(pos.translation.y + timeline_size.y / 2.);
            entity_commands
                .insert((InheritedVisibility::VISIBLE, layer.clone()))
                .with_child((
                    Camera2d,
                    layer.clone(),
                    Camera {
                        order: entity.index_u32() as isize,
                        viewport: Some(Viewport {
                            physical_position: (main_camera
                                .0
                                .world_to_viewport(main_camera.1, viewport_pos)
                                .unwrap()
                                * window.scale_factor())
                            .as_uvec2(),
                            physical_size: (timeline_size * window.scale_factor()).as_uvec2(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ));

            // Main, horizontal line
            entity_commands.with_child((
                Mesh2d(meshes.add(Rectangle::new(timeline_size.x, 3.))),
                MeshMaterial2d(materials.add(Color::srgb(0.9, 0.9, 0.9))),
                pos.with_translation(Vec3::ZERO),
                layer.clone(),
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

                entity_commands.with_child((
                    Mesh2d(year_line_mesh.clone()),
                    MeshMaterial2d(year_line_material.clone()),
                    Transform::from_xyz(year_x_pos, 0., 0.),
                    layer.clone(),
                    children![(
                        Text2d::new(year_iterator.next().unwrap().to_string()),
                        Transform::from_xyz(0., -15., 0.),
                        layer.clone()
                    )],
                ));
            }
        }
    }
}
