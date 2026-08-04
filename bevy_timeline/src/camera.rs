// BEWARE THEE WHO LOOKEST INTO THIS MODULE:
// IT CONTAINS THE MOST CURSED CODE THAT WAS FORSAKEN BY THE GODS AND IS PROBABLY INCORRECT IN MULTIPLE EDGE CASES
// BE YE NOT FOOLISH TO MESS WITH IT, LEST YOUR SOUL AND YOUR DAYS MIGHT BE SUCKED AWAY BY THE EVIL SPIRIT OF THE CAMERAS

use bevy::{
    camera::{
        CameraProjection as _, ComputedCameraValues, RenderTargetInfo, Viewport,
        visibility::RenderLayers,
    },
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

#[derive(Debug, Component)]
struct TimelineCamera(Entity);

/// THIS CURSED STRUCT MAY NOT BE REMOVED, FOR IT CONTAINS THAT WHICH IS ESSENTIAL TO A CONSISTENT TIMELINE PRESENTATION UPON WINDOW RESIZES.
///
/// This is used to nudge the [`TimelineCamera`]'s position so that its viewport doesn't go out of bounds. Since the viewport's physical position is a [`UVec2`],
/// negative positions are set to 0 and viewports would be too big and show hidden parts of the timeline. If we only adjust the physical size as well, the perspective
/// on the timeline is incorrect and shows a different part than what should be shown.
#[derive(Debug, Default)]
struct CutoffAmount(Vec2);

impl TimelineCameraPlugin {
    fn make_viewport(
        render_size: Vec2,
        pos: &Transform,
        window: &Window,
    ) -> Option<(Viewport, CutoffAmount)> {
        let physical_pos = {
            // The viewport position is on the top-left corner. In order to convert the pos translation (which has (0,0) at the center) to that,
            // we need to move the coords left and up, which due to Bevy's coordinate system means adding y and subtracting x.
            let logical_pos = Vec3 {
                x: pos.translation.x - render_size.x / 2.,
                y: pos.translation.y + render_size.y / 2.,
                z: pos.translation.z,
            };

            // Taken loosely from bevy internals... We need a camera to calculate the viewport pos.
            let projection_camera = {
                let mut projection = OrthographicProjection::default_2d();
                projection.update(window.width(), window.height());
                let clip_from_view = projection.get_clip_from_view();

                Camera {
                    computed: ComputedCameraValues {
                        target_info: Some(RenderTargetInfo {
                            physical_size: window.physical_size(),
                            scale_factor: window.scale_factor(),
                        }),
                        clip_from_view,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            };

            projection_camera
                .world_to_viewport(&GlobalTransform::IDENTITY, logical_pos)
                .expect("The main camera's coordinates are convertible to a viewport")
                * window.scale_factor()
        };

        let mut physical_size = render_size * window.scale_factor();

        let cutoff_amount = physical_pos.map(|e| e.min(0.));
        physical_size += cutoff_amount;

        let physical_pos = physical_pos.as_uvec2();

        if physical_size.min_element() <= 0.
            || physical_pos.x > window.physical_width()
            || physical_pos.y > window.physical_height()
        {
            return None;
        }
        Some((
            Viewport {
                physical_position: physical_pos,
                physical_size: physical_size.as_uvec2(),
                ..Default::default()
            },
            CutoffAmount(cutoff_amount / window.scale_factor()),
        ))
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

            let (viewport, cutoff) = Self::make_viewport(render_size, pos, &window)
                .map_or(Default::default(), |(v, c)| (Some(v), c));

            commands.spawn((
                pos.to_owned()
                    .with_translation(cutoff.adjust_translation(pos.translation)),
                TimelineCamera(timeline_entity),
                Camera2d,
                render_layers.clone(),
                Camera {
                    order: render_layers
                        .iter()
                        .next()
                        .expect("There should be a render layer for the Timeline")
                        .try_into()
                        .expect("RenderLayer shouldn't be huge"),
                    is_active: viewport.is_some(),
                    viewport,

                    ..Default::default()
                },
            ));
        }
    }

    fn resize_timeline_cameras(
        timeline_info_query: Query<
            (
                Option<&crate::configuration::TimelineScreenSize>,
                &Transform,
            ),
            Without<TimelineCamera>,
        >,
        mut cameras: Query<(&mut Camera, &mut Transform, &TimelineCamera)>,
        window: Single<&Window, With<PrimaryWindow>>,
    ) {
        for (mut cam, mut cam_pos, &TimelineCamera(timeline)) in cameras.iter_mut() {
            let (size, pos) = timeline_info_query
                .get(timeline)
                .expect("Timeline Camera should refer to a RenderedTimeline");
            let render_size = size.map_or(window.size(), |s| **s);
            if let Some((viewport, cutoff)) = Self::make_viewport(render_size, pos, &window) {
                cam.viewport = Some(viewport);
                cam.is_active = true;
                // Adjust the camera position so the perspective from the timeline isn't weird and nothing hidden is shown.
                cam_pos.translation = cutoff.adjust_translation(pos.translation);
            } else {
                cam.is_active = false;
            }
        }
    }
}

impl CutoffAmount {
    fn adjust_translation(&self, translation: Vec3) -> Vec3 {
        translation + self.0.extend(0.).with_x(-self.0.x) / 2.
    }
}
