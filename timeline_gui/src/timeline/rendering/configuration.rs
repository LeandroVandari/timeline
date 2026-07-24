use core::sync::atomic::AtomicUsize;

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
/// Possible configuration values are available in [`timeline::rendering::configuration`](crate::timeline::rendering::configuration).
#[derive(Debug, Component, Default)]
#[require(
    Transform,
    Timeline,
    InheritedVisibility,
    TimelineRenderRange(YearRange {start: Year::current().unwrap() - 20, end: Year::current().unwrap() + 20}),
    TimelineLineSeparation(100.),
    TimelineHorizontalOffset(0.),
    TimelineVerticalOffset(0.),
    RenderLayers = next_render_layer(),
    crate::zooming::ZoomLevel
)]
pub struct RenderedTimeline;

fn next_render_layer() -> RenderLayers {
    RenderLayers::layer(TIMELINE_RENDER_LAYER.fetch_add(1, core::sync::atomic::Ordering::Relaxed))
}

#[instrument(skip_all)]
fn add_rendered_timeline(mut world: DeferredWorld, ctx: HookContext) {
    world
        .commands()
        .entity(ctx.entity)
        .insert_if_new(RenderedTimeline);
}

/// Setting for a [`RenderedTimeline`] that indicates how spaced apart the vertical lines should be.
#[derive(Debug, Component, Deref, Clone, Copy, Default)]
#[component(on_add = add_rendered_timeline)]
pub struct TimelineLineSeparation(pub f32);

/// How much space the [`RenderedTimeline`] should occupy on screen. If not present, renderer will default to window size.
#[derive(Debug, Component, Deref, Clone, Copy)]
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

#[derive(Debug, Component, Clone, Copy, Deref, DerefMut)]
#[component(on_add = add_rendered_timeline)]
pub struct TimelineHorizontalOffset(f32);

#[derive(Debug, Component, Clone, Copy, Deref, DerefMut)]
#[component(on_add = add_rendered_timeline)]
pub struct TimelineVerticalOffset(f32);

impl TimelineRenderRange {
    pub fn inc(&mut self) {
        self.0.end = self.0.end.get_next().unwrap();
        self.0.start = self
            .0
            .start
            .get_next()
            .expect("since start < end, and end has a next, start should also.");
    }

    pub fn dec(&mut self) {
        self.0.start = self.0.start.get_previous().unwrap();
        self.0.end = self
            .0
            .end
            .get_previous()
            .expect("since start < end, and start has a previous, end should also.");
    }
}

#[derive(Debug, Message)]
pub struct RenderedTimelineCreatedMessage(Entity);

impl RenderedTimelineCreatedMessage {
    #[must_use]
    pub const fn entity(&self) -> Entity {
        self.0
    }

    #[must_use]
    pub fn from_trigger(trigger: On<Add, RenderedTimeline>) -> Self {
        Self(trigger.entity)
    }
}
