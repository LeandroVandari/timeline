use crate::RenderedTimeline;
use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};
use tracing::instrument;

mod line_separation;
mod offsets;
mod render_range;
mod screen_size;

pub use line_separation::TimelineLineSeparation;
pub use offsets::{TimelineHorizontalOffset, TimelineVerticalOffset};
pub use render_range::TimelineRenderRange;
pub use screen_size::TimelineScreenSize;

#[instrument(skip_all)]
fn add_rendered_timeline(mut world: DeferredWorld, ctx: HookContext) {
    world
        .commands()
        .entity(ctx.entity)
        .insert_if_new(RenderedTimeline::default());
}
