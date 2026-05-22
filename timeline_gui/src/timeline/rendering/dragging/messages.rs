use bevy::prelude::*;

#[derive(Debug, Message)]
pub struct DragMessage {
    pub(super) dragged_entity: Entity,
    pub(super) delta: Vec2,
}

impl DragMessage {
    pub fn new(dragged_entity: Entity, delta: Vec2) -> Self {
        Self {
            dragged_entity,
            delta,
        }
    }
}
