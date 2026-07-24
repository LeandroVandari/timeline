use bevy::camera::Viewport;
use bevy::camera::visibility::RenderLayers;
#[cfg(target_os = "macos")]
use bevy::input::gestures::PinchGesture;
use bevy::log::tracing::instrument;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::dragging::DraggingPlugin;
use crate::setup::MainCamera;
use crate::timeline::rendering::configuration::{TimelineLineSeparation, TimelineRenderRange};
use crate::timeline::rendering::lines::TimelineLinesPlugin;
use crate::wrap_around::WrapAroundPlugin;
use crate::zooming::{ZoomLevel, ZoomingPlugin};
pub use configuration::RenderedTimeline;
use configuration::RenderedTimelineCreatedMessage;

mod background;
pub mod configuration;

mod lines;

pub struct TimelineRendererPlugin;

impl Plugin for TimelineRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                Self::spawn_timeline_camera,
                background::spawn_timeline_background,
            )
                .run_if(on_message::<RenderedTimelineCreatedMessage>),
        )
        .add_observer(
            |trigger: On<Add, RenderedTimeline>,
             mut writer: MessageWriter<RenderedTimelineCreatedMessage>| {
                info!(
                    "Spawning rendering components for timeline {}",
                    trigger.entity
                );

                writer.write(RenderedTimelineCreatedMessage::from_trigger(trigger));
            },
        )
        .add_message::<RenderedTimelineCreatedMessage>()
        .add_plugins((
            DraggingPlugin,
            ZoomingPlugin,
            TimelineLinesPlugin,
            WrapAroundPlugin,
        ));

        #[cfg(target_os = "macos")]
        app.add_systems(
            Update,
            background::emit_timeline_zoom_message_on_pinch.run_if(on_message::<PinchGesture>),
        );
    }
}

#[expect(
    clippy::cast_precision_loss,
    reason = "Will only lose precision for extreme ranges"
)]
#[must_use]
pub fn draw_width(
    render_range: &TimelineRenderRange,
    line_separation: TimelineLineSeparation,
    zoom: ZoomLevel,
) -> f32 {
    (render_range.0.len() - 1) as f32 * *line_separation * *zoom
}

impl TimelineRendererPlugin {
    /// Creates a new camera to render the timeline whose [`RenderedTimeline`] was just spawned.
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

        timeline_info_query: Query<(
            Option<&configuration::TimelineScreenSize>,
            &Transform,
            &RenderLayers,
        )>,

        mut added_render_infos: MessageReader<RenderedTimelineCreatedMessage>,
    ) {
        for msg in added_render_infos.read() {
            let entity = msg.entity();
            let (size, pos, render_layers) = timeline_info_query.get(entity).expect("The message is only called with an entity that has RenderedTimeline, and thus its required components.");
            trace!("Spawning camera for timeline {entity}");

            let render_size = size.map_or(window.size(), |s| **s);

            // The viewport position is on the top-left corner. In order to convert the pos translation (which has (0,0) at the center) to that,
            // we need to move the coords left and up, which due to Bevy's coordinate system means adding y and subtracting x.
            let viewport_pos = pos
                .translation
                .with_x(pos.translation.x - render_size.x / 2.)
                .with_y(pos.translation.y + render_size.y / 2.);
            commands.entity(entity).with_child((
                Camera2d,
                render_layers.clone(),
                Camera {
                    order: render_layers
                        .iter()
                        .next()
                        .expect("There should be a render layer for the Timeline")
                        .try_into()
                        .expect("RenderLayer shouldn't be huge"),
                    viewport: Some(Viewport {
                        physical_position: (main_camera
                            .0
                            .world_to_viewport(main_camera.1, viewport_pos)
                            .expect("The main camera's coordinates are convertible to a viewport")
                            * window.scale_factor())
                        .as_uvec2(),
                        physical_size: (render_size * window.scale_factor()).as_uvec2(),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ));
        }
    }
}
