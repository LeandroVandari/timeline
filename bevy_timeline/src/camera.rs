use bevy::{
    camera::{ComputedCameraValues, RenderTargetInfo, Viewport, visibility::RenderLayers},
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};

use crate::message::RenderedTimelineCreatedMessage;

pub struct TimelineCameraPlugin;

impl Plugin for TimelineCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                Self::spawn_timeline_camera,
                Self::resize_timeline_cameras.run_if(on_message::<WindowResized>),
            ),
        );
    }
}

impl TimelineCameraPlugin {
    fn make_viewport(render_size: Vec2, pos: &Transform, window: &Window) -> Viewport {
        // The viewport position is on the top-left corner. In order to convert the pos translation (which has (0,0) at the center) to that,
        // we need to move the coords left and up, which due to Bevy's coordinate system means adding y and subtracting x.
        let viewport_pos = pos
            .translation
            .with_x(pos.translation.x - render_size.x / 2.)
            .with_y(pos.translation.y + render_size.y / 2.);

        Viewport {
            physical_position: (Camera {
                computed: ComputedCameraValues {
                    target_info: Some(RenderTargetInfo {
                        physical_size: window.physical_size(),
                        scale_factor: window.scale_factor(),
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }
            .world_to_viewport(&GlobalTransform::IDENTITY, viewport_pos)
            .expect("The main camera's coordinates are convertible to a viewport")
                * window.scale_factor())
            .as_uvec2(),
            physical_size: (render_size * window.scale_factor()).as_uvec2(),
            ..Default::default()
        }
    }

    /// Creates a new camera to render the timeline whose [`RenderedTimeline`](crate::RenderedTimeline) was just spawned.
    ///
    /// We need to make sure only the proper area from the timeline is drawn by creating a custom camera just to render it whose viewport spans exactly
    /// the size of the timeline.
    /// This is needed so that events/labels can get cut off rather than popping into existance.
    ///
    /// I'm not sure of the performance implications of multiple cameras, but I'm pretty sure it's not too bad.
    fn spawn_timeline_camera(
        mut commands: Commands,

        window: Single<&Window, With<PrimaryWindow>>,

        timeline_info_query: Query<(
            Option<&crate::configuration::TimelineScreenSize>,
            &Transform,
            &RenderLayers,
        )>,

        mut new_rendered_timelines: PopulatedMessageReader<RenderedTimelineCreatedMessage>,
    ) {
        for msg in new_rendered_timelines.read() {
            let timeline_entity = msg.entity();
            let (size, pos, render_layers) = timeline_info_query.get(timeline_entity).expect("The message is only called with an entity that has RenderedTimeline, and thus its required components.");
            trace!("Spawning camera for timeline {timeline_entity}");

            let render_size = size.map_or(window.size(), |s| **s);

            commands.spawn((
                ChildOf(timeline_entity),
                Camera2d,
                render_layers.clone(),
                Camera {
                    order: render_layers
                        .iter()
                        .next()
                        .expect("There should be a render layer for the Timeline")
                        .try_into()
                        .expect("RenderLayer shouldn't be huge"),
                    viewport: Some(Self::make_viewport(render_size, pos, &window)),
                    ..Default::default()
                },
            ));
        }
    }

    fn resize_timeline_cameras(
        timeline_info_query: Query<(
            Option<&crate::configuration::TimelineScreenSize>,
            &Transform,
        )>,
        mut cameras: Query<(&mut Camera, &ChildOf)>,
        window: Single<&Window, With<PrimaryWindow>>,
    ) {
        for (mut cam, &ChildOf(parent)) in cameras.iter_mut() {
            let Ok((size, pos)) = timeline_info_query.get(parent) else {
                continue;
            };
            let render_size = size.map_or(window.size(), |s| **s);
            let Some(viewport) = cam.viewport.as_mut() else {
                error!("RenderedTimeline Camera has no Viewport!");
                return;
            };

            *viewport = Self::make_viewport(render_size, pos, &window);
        }
    }
}
