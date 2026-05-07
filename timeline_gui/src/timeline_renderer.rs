use bevy::camera::Viewport;
use bevy::camera::visibility::RenderLayers;
use bevy::log::tracing::instrument;
use bevy::{prelude::*, window::PrimaryWindow};
use timeline_core::date_iteration::YearIterator;

use crate::setup::MainCamera;
use crate::timeline::render_information::{
    TimelineRenderInformation, TimelineRenderInformationCreatedMessage,
};

pub struct TimelineRendererPlugin;

impl Plugin for TimelineRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (Self::spawn_timeline_camera, Self::spawn_timeline_lines)
                .run_if(on_message::<TimelineRenderInformationCreatedMessage>),
        )
        .add_message::<TimelineRenderInformationCreatedMessage>();
    }
}

impl TimelineRendererPlugin {
    /// Creates a new camera to render the timeline whose [`TimelineRenderInformation`] was just spawned.
    ///
    /// We need to make sure only the proper area from the timeline is drawn by creating a custom camera just to render it whose viewport spans exactly
    /// the size of the timeline.
    /// This is needed so that events/labels can get cut off rather than popping into existance.
    ///
    /// I'm not sure of the performance implications of multiple cameras, but I'm pretty sure it's not too bad.
    #[instrument(skip_all)]
    fn spawn_timeline_camera(
        mut commands: Commands,
        main_camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
        window: Single<&Window, With<PrimaryWindow>>,
        render_info_query: Query<(&TimelineRenderInformation, &Transform)>,
        mut added_render_infos: MessageReader<TimelineRenderInformationCreatedMessage>,
    ) {
        for msg in added_render_infos.read() {
            let entity = msg.entity();
            let (render_info, pos) = render_info_query.get(entity).expect("The message is only called with an entity that has TimelineRenderInformation, and that requires Transform");
            info!(
                "Spawning camera for timeline {} with render configuration {:#?}",
                entity, render_info
            );
            let timeline_size = render_info.size.unwrap_or(window.size());
            let render_layer = Self::get_render_layer(entity);

            // The viewport position is on the top-left corner. In order to convert the pos translation (which has (0,0) at the center) to that,
            // we need to move the coords left and up, which due to Bevy's coordinate system means adding y and subtracting x.
            let viewport_pos = pos
                .translation
                .with_x(pos.translation.x - timeline_size.x / 2.)
                .with_y(pos.translation.y + timeline_size.y / 2.);
            commands
                .entity(entity)
                // Make sure the entity is in the same render layer
                .insert(render_layer.clone())
                .with_child((
                    Camera2d,
                    render_layer,
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
        }
    }

    /// Spawn the lines for each ear and corresponding labels for drawing the timelines.
    #[instrument(skip_all)]
    fn spawn_timeline_lines(
        mut commands: Commands,
        window: Single<&Window, With<PrimaryWindow>>,

        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,

        render_info_query: Query<(&TimelineRenderInformation, &Transform)>,
        mut added_render_infos: MessageReader<TimelineRenderInformationCreatedMessage>,
    ) {
        for msg in added_render_infos.read() {
            let entity = msg.entity();
            let (render_info, pos) = render_info_query
                .get(entity)
                .expect("Message should refer to an entity with proper components.");
            info!(
                "Spawning lines for timeline {} with render configuration {:#?}",
                entity, render_info
            );
            let timeline_size = render_info.size.unwrap_or(window.size());
            let render_layer = Self::get_render_layer(entity);

            // Main, horizontal line
            commands.entity(entity).with_child((
                Mesh2d(meshes.add(Rectangle::new(timeline_size.x, 3.))),
                MeshMaterial2d(materials.add(Color::srgb(0.9, 0.9, 0.9))),
                pos.with_translation(Vec3::ZERO),
                render_layer.clone(),
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
                    render_layer.clone(),
                    children![(
                        Text2d::new(year_iterator.next().unwrap().to_string()),
                        Transform::from_xyz(0., -15., 0.),
                        render_layer.clone()
                    )],
                ));
            }
        }
    }

    /// Gets the render layer the timeline will be rendered at.
    fn get_render_layer(entity: Entity) -> RenderLayers {
        RenderLayers::layer(entity.index_u32() as usize)
    }
}
