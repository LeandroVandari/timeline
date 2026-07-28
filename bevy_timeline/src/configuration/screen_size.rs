use bevy::prelude::*;

use crate::RenderedTimeline;

/// How much space the [`RenderedTimeline`] should occupy on screen. If not present, renderer will default to window size.
#[derive(Debug, Component, Deref, Clone, Copy)]
// We can require because it doesn't cause a cycle since TimelineScreenSize is optional.
#[require(RenderedTimeline)]
pub struct TimelineScreenSize(pub Vec2);
