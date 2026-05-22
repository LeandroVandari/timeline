use std::sync::atomic::AtomicUsize;

use bevy::{
    camera::visibility::RenderLayers,
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};
use timeline_core::date_iteration::year::Year;
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
    TimelineStartYear(Year::current().unwrap()),
    TimelineLineSeparation(100.),
    TimelineHorizontalRenderMargin(0.5),
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

/// Setting for a [`RenderedTimeline`] that indicates the leftmost rendered year.
#[derive(Debug, Component, Deref, DerefMut, Clone)]
#[component(on_add = add_rendered_timeline)]
pub struct TimelineStartYear(pub Year);

/// Setting for a [`RenderedTimeline`] that indicates how spaced apart the vertical lines should be.
#[derive(Debug, Component, Deref, DerefMut, Clone, Copy, Default)]
#[component(on_add = add_rendered_timeline)]
pub struct TimelineLineSeparation(pub f32);

/// How much space the [`RenderedTimeline`] should occupy. If not present, renderer will default to window size.
#[derive(Debug, Component, Deref, DerefMut, Clone, Copy)]
// We can require because it doesn't cause a cycle since TimelineSize is optional.
#[require(RenderedTimeline)]
pub struct TimelineSize(pub Vec2);

/// How much extra invisible space the [`RenderedTimeline`] should render beyond the visible area, in percent.
///
/// This is needed so that things that have a width don't just pop into existence half way into the screen when their coordinate gets into the visible area.
#[derive(Debug, Component, Deref, DerefMut, Clone, Copy)]
#[component(on_add=add_rendered_timeline)]
pub struct TimelineHorizontalRenderMargin(pub f32);

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
