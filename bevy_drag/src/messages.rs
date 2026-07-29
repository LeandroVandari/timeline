use bevy::prelude::*;

/// Message that represents a drag event for a group of entities.
///
/// When [`DragMessage`] is emitted by a system, all entities with a [`*DraggedBy`](super::DraggedBy) component
/// pointing to [`drag_entity`](Self::drag_entity)'s will have their
/// [`Transform`] correspondingly updated in the next [`Update`] schedule, in a system provided by [`DraggingPlugin`](super::DraggingPlugin).
#[derive(Debug, Message)]
pub struct DragMessage {
    /// The [`Entity`] that was dragged. All entities [`dragged by`](super::DraggedBy) it will
    /// have their [`Transform`] correspondingly updated.
    pub drag_entity: Entity,
    /// How much the dragged entities' [`Transform`] should be updated by.
    pub delta: Vec2,
}
