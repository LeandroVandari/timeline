use bevy::prelude::*;

/// Setting for a [`RenderedTimeline`] that indicates how spaced apart the vertical lines should be.
#[derive(Debug, Component, Deref, Clone, Copy, Default)]
#[component(on_add = super::add_rendered_timeline)]
pub struct TimelineLineSeparation(pub f32);
