use bevy::prelude::*;
use timeline_core::TimelineManager;

pub mod render_information;

#[derive(Debug, Component, Default)]
/// A historical timeline to be held by the Bevy ECS.
/// Will not be rendered unless its entity cotains [`TimelineRenderInformation`](render_information::TimelineRenderInformation).
pub struct Timeline {
    pub manager: TimelineManager,
}
