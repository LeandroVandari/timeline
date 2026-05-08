use std::sync::atomic::AtomicUsize;

use bevy::{camera::visibility::RenderLayers, prelude::*};
use timeline_core::date_iteration::year::Year;

use crate::timeline::Timeline;

static TIMELINE_RENDER_LAYER: AtomicUsize = AtomicUsize::new(1);

/// Information that describes how a [`Timeline`] should be rendered.
#[derive(Debug, Component)]
#[require(Transform, Timeline, InheritedVisibility)]
pub struct TimelineRenderInformation {
    /// Leftmost year rendered.
    pub year_start: Year,
    /// How much the years/vertical lines should be moved horizontally
    /// relative to the leftmost position of the [`Timeline`].
    pub horizontal_offset: f32,
    /// How spaced apart the year lines should be.
    pub line_dist: f32,
    /// How much space the rendered [`Timeline`] should occupy. Should default to the maximum available if [None].
    pub size: Option<Vec2>,
    /// Which layer the [`Timeline`] should be rendered in. [Default] impl uses next number in a monotonically increasing sequence
    /// if not specified.
    ///
    /// Since a *new* [`Camera`] will be spawned with these layers, and there can't be more than one [`Camera`] rendering the same layer, this should be left
    /// for [Default] to fill.
    pub layers: RenderLayers,
}
#[derive(Debug, Message)]
pub struct TimelineRenderInformationCreatedMessage(Entity);

impl TimelineRenderInformationCreatedMessage {
    pub const fn entity(&self) -> Entity {
        self.0
    }

    pub fn from_trigger(trigger: On<Add, TimelineRenderInformation>) -> Self {
        Self(trigger.entity)
    }
}

impl Default for TimelineRenderInformation {
    /// Default rendering options for a [`Timeline`](super::Timeline).
    /// Starts from the current year, with a separation of 100px between lines.
    fn default() -> Self {
        Self {
            year_start: Year::current().unwrap(),
            horizontal_offset: 50.,
            line_dist: 100.,
            size: None,
            layers: RenderLayers::layer(
                TIMELINE_RENDER_LAYER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            ),
        }
    }
}
