use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};
use timeline_core::date_iteration::year::Year;

/// Information that describes how a [`Timeline`] should be rendered.
#[derive(Debug, Component)]
#[require(Transform, super::Timeline, InheritedVisibility)]
#[component(on_add = Self::emit_message_added)]
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
}

impl TimelineRenderInformation {
    fn emit_message_added(mut world: DeferredWorld, ctx: HookContext) {
        world.write_message(TimelineRenderInformationCreatedMessage(ctx.entity));
    }
}

#[derive(Debug, Message)]
pub struct TimelineRenderInformationCreatedMessage(Entity);

impl TimelineRenderInformationCreatedMessage {
    pub fn entity(&self) -> Entity {
        self.0
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
        }
    }
}
