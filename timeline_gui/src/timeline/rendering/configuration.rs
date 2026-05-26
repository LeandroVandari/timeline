use std::sync::atomic::AtomicUsize;

use bevy::{
    camera::visibility::RenderLayers,
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};
use timeline_core::date_iteration::{YearRange, year::Year};
use tracing::instrument;

use crate::timeline::Timeline;

static TIMELINE_RENDER_LAYER: AtomicUsize = AtomicUsize::new(1);

/// Component that indicates a [`Timeline`] should be rendered to the screen.
///
/// Possible configuration values are available in [timeline::rendering::configuration](crate::timeline::rendering::configuration).
#[derive(Debug, Component, Default)]
#[require(
    Transform,
    Timeline,
    InheritedVisibility,
    TimelineRenderRange(YearRange {start: Year::current().unwrap() - 20, end: Year::current().unwrap() + 20}),
    TimelineLineSeparation(100.),
    RenderLayers = next_render_layer()
)]
pub struct RenderedTimeline;

fn next_render_layer() -> RenderLayers {
    RenderLayers::layer(TIMELINE_RENDER_LAYER.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
}

#[instrument(skip_all)]
fn add_rendered_timeline(mut world: DeferredWorld, ctx: HookContext) {
    world
        .commands()
        .entity(ctx.entity)
        .insert_if_new(RenderedTimeline);
}

/// Setting for a [`RenderedTimeline`] that indicates how spaced apart the vertical lines should be.
#[derive(Debug, Component, Deref, DerefMut, Clone, Copy, Default)]
#[component(on_add = add_rendered_timeline)]
pub struct TimelineLineSeparation(pub f32);

/// How much space the [`RenderedTimeline`] should occupy on screen. If not present, renderer will default to window size.
#[derive(Debug, Component, Deref, DerefMut, Clone, Copy)]
// We can require because it doesn't cause a cycle since TimelineScreenSize is optional.
#[require(RenderedTimeline)]
pub struct TimelineScreenSize(pub Vec2);

/// The range of years the [`RenderedTimeline`] will render.
///
/// Note that this doesn't mean all rendered years appear on screen. The actual visible year range depends
/// on the [`TimelineScreenSize`] and [`TimelineLineSeparation`]
#[derive(Debug, Component, Clone)]
#[component(on_add = add_rendered_timeline)]
pub struct TimelineRenderRange(pub YearRange);

impl TimelineLineSeparation {
    #[allow(unused)]
    pub fn from_range_and_width(range: &TimelineRenderRange, width: f32) -> Self {
        Self(width / (range.0.len() - 1) as f32)
    }
}

impl TimelineRenderRange {
    pub fn inc(&mut self) {
        self.0.start = self.0.start.get_next().unwrap();
        self.0.end = self.0.end.get_next().unwrap();
    }

    pub fn dec(&mut self) {
        self.0.start = self.0.start.get_previous().unwrap();
        self.0.end = self.0.end.get_previous().unwrap();
    }
}

#[derive(Debug, Message)]
pub struct RenderedTimelineCreatedMessage(Entity);

impl RenderedTimelineCreatedMessage {
    pub const fn entity(&self) -> Entity {
        self.0
    }

    pub fn from_trigger(trigger: On<Add, RenderedTimeline>) -> Self {
        Self(trigger.entity)
    }
}
