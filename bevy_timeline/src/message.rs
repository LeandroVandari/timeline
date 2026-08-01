use bevy::prelude::*;

#[derive(Debug, Message)]
pub struct RenderedTimelineCreatedMessage(Entity);

impl RenderedTimelineCreatedMessage {
    #[must_use]
    pub const fn entity(&self) -> Entity {
        self.0
    }

    #[must_use]
    pub fn from_trigger(trigger: On<Add, crate::RenderedTimeline>) -> Self {
        Self(trigger.entity)
    }
}
