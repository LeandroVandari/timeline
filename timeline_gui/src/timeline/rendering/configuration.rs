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
    TimelineHorizontalOffset,
    TimelineStartYear,
    TimelineLineSeparation,
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

/// Setting for a [`RenderedTimeline`] that describes how much the rendered elements should be moved horizontally.
///
/// This is used to track e.g. when vertical year lines and year labels can be reutilized to wrap around and move to the other end of the timeline.
///
/// As an initial setting, this is useful so the timeline doesn't look unnatural with the leftmost line hugging the screen edge.
#[derive(Debug, Component, Deref, DerefMut, Clone, Copy)]
#[component(on_add = add_rendered_timeline)]
pub struct TimelineHorizontalOffset(pub f32);

/// Setting for a [`RenderedTimeline`] that indicates the leftmost rendered year.
#[derive(Debug, Component, Deref, DerefMut, Clone)]
#[component(on_add = add_rendered_timeline)]
pub struct TimelineStartYear(pub Year);

/// Setting for a [`RenderedTimeline`] that indicates how spaced apart the vertical lines should be.
#[derive(Debug, Component, Deref, DerefMut, Clone, Copy)]
#[component(on_add = add_rendered_timeline)]
pub struct TimelineLineSeparation(pub f32);

/// How much space the [`RenderedTimeline`] should occupy. If not present, renderer will default to window size.
#[derive(Debug, Component, Deref, DerefMut, Clone, Copy)]
#[require(RenderedTimeline)]
pub struct TimelineSize(pub Vec2);

impl Default for TimelineHorizontalOffset {
    fn default() -> Self {
        Self(50.)
    }
}
impl Default for TimelineStartYear {
    fn default() -> Self {
        Self(Year::current().unwrap())
    }
}
impl Default for TimelineLineSeparation {
    fn default() -> Self {
        Self(100.)
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
