#![expect(clippy::needless_pass_by_value, reason = "Bevy Queries")]

use core::sync::atomic::AtomicUsize;

use bevy::{camera::visibility::RenderLayers, prelude::*};
use timeline_core::{
    TimelineManager,
    date_iteration::{YearRange, year::Year},
};

pub mod configuration;
#[cfg(feature = "debug")]
pub mod debug;
mod input;
mod lines;
mod message;
mod plugin;

pub use plugin::TimelineRendererPlugin;

static TIMELINE_RENDER_LAYER: AtomicUsize = AtomicUsize::new(1);

/// Component that indicates a [`Timeline`] should be rendered to the screen.
///
/// Possible configuration values are available in [`timeline::rendering::configuration`](crate::timeline::rendering::configuration).
#[derive(Debug, Component, Default)]
#[require(
    Transform,
    InheritedVisibility,
    configuration::TimelineRenderRange(YearRange {start: Year::current().unwrap() - 20, end: Year::current().unwrap() + 20}),
    configuration::TimelineLineSeparation(100.),
    configuration::TimelineHorizontalOffset::ZERO,
    configuration::TimelineVerticalOffset::ZERO,
    RenderLayers = Self::next_render_layer(),
    bevy_zoom::ZoomLevel
)]
pub struct RenderedTimeline {
    manager: TimelineManager,
}

impl RenderedTimeline {
    #[must_use]
    pub fn new(manager: TimelineManager) -> Self {
        Self { manager }
    }

    fn next_render_layer() -> RenderLayers {
        RenderLayers::layer(
            TIMELINE_RENDER_LAYER.fetch_add(1, core::sync::atomic::Ordering::Relaxed),
        )
    }

    #[expect(
        clippy::cast_precision_loss,
        reason = "Will only lose precision for extreme ranges"
    )]
    #[must_use]
    pub fn draw_width(
        render_range: &configuration::TimelineRenderRange,
        line_separation: configuration::TimelineLineSeparation,
        zoom: bevy_zoom::ZoomLevel,
    ) -> f32 {
        (render_range.0.len() - 1) as f32 * *line_separation * *zoom
    }
}
