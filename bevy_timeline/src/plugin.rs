use bevy::prelude::*;

use crate::{RenderedTimeline, message::RenderedTimelineCreatedMessage};

pub struct TimelineRendererPlugin;

impl Plugin for TimelineRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(
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
            bevy_drag::DraggingPlugin,
            bevy_zoom::ZoomingPlugin,
            crate::lines::TimelineLinesPlugin,
            bevy_wrap::WrapAroundPlugin,
            crate::input::TimelineInputHandlerPlugin,
            crate::camera::TimelineCameraPlugin,
        ));
    }
}
