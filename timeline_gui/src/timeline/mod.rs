use bevy::prelude::*;
use timeline_core::TimelineManager;

pub mod rendering;
pub use rendering::RenderedTimeline;

#[derive(Debug, Component, Default)]
/// A historical timeline to be held by the Bevy ECS.
/// Will not be rendered unless its entity contains [`RenderedTimeline`].
pub struct Timeline {
    pub manager: TimelineManager,
}
