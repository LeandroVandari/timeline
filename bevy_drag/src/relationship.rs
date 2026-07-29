use bevy::prelude::*;

/// [`RelationshipTarget`] that relates to all entities with a [`VerticallyDraggedBy`] [`Component`] pointing to this [`Entity`].
///
/// This cannot be manually spawned. It is left as a public item just to allow for integration with other systems that might
/// want the same information. It can be [`Query`]ed as a normal [`Component`].
///
/// See more information in the [crate root](crate).
#[derive(Debug, Component)]
#[relationship_target(relationship=VerticallyDraggedBy)]
pub struct VerticallyDrags(Vec<Entity>);

/// Entities with this component will update their vertical translation when a [`DragMessage`](super::DragMessage) for the
/// corresponding [`Entity`] is emmited.
///
/// See more information in the [crate root](crate).
#[derive(Debug, Component)]
#[relationship(relationship_target=VerticallyDrags)]
pub struct VerticallyDraggedBy(pub Entity);

/// [`RelationshipTarget`] that relates to all entities with a [`HorizontallyDraggedBy`] [`Component`] pointing to this [`Entity`].
///
/// This cannot be manually spawned. It is left as a public item just to allow for integration with other systems that might
/// want the same information. It can be [`Query`]ed as a normal [`Component`].
///
/// See more information in the [crate root](crate).
#[derive(Debug, Component)]
#[relationship_target(relationship=HorizontallyDraggedBy)]
pub struct HorizontallyDrags(Vec<Entity>);

/// Entities with this component will update their horizontal translation when a [`DragMessage`](super::DragMessage) for the
/// corresponding [`Entity`] is emmited.
///
/// See more information in the [crate root](crate).
#[derive(Debug, Component)]
#[relationship(relationship_target=HorizontallyDrags)]
pub struct HorizontallyDraggedBy(pub Entity);

/// Convenience [`Bundle`] that represents entities that should be both horizontally and vertically dragged by the target [`Entity`].
///
/// Entities with [`DraggedBy`] will have their [`Transform`]s updated by [`DragMessage`](super::DragMessage)s targeting the corresponding entity.
#[derive(Debug, Bundle)]
pub struct DraggedBy(VerticallyDraggedBy, HorizontallyDraggedBy);

impl DraggedBy {
    /// Create a new [`DraggedBy`] bundle that targets `entity`.
    #[must_use]
    pub fn new(entity: Entity) -> Self {
        Self(VerticallyDraggedBy(entity), HorizontallyDraggedBy(entity))
    }
}
