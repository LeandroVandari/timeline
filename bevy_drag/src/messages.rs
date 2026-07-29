use bevy::prelude::*;

#[derive(Debug, Message)]
pub struct DragMessage {
    pub drag_entity: Entity,
    pub delta: Vec2,
}
