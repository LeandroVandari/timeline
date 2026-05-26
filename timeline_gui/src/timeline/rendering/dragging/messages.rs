use bevy::prelude::*;

#[derive(Debug, Message)]
pub struct DragMessage {
    dragged_entity: Entity,
    delta: Vec2,
}

impl DragMessage {
    #[must_use]
    pub fn new(dragged_entity: Entity, delta: Vec2) -> Self {
        Self {
            dragged_entity,
            delta,
        }
    }

    #[must_use]
    pub fn entity(&self) -> Entity {
        self.dragged_entity
    }

    #[must_use]
    pub fn delta(&self) -> Vec2 {
        self.delta
    }
}

#[derive(Debug, EntityEvent)]
pub struct WrapAround {
    pub entity: Entity,
    pub direction: WrapDirection,
}

#[derive(Debug)]
pub enum WrapDirection {
    Left,
    Right,
}
