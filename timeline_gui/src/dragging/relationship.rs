use bevy::prelude::*;

#[derive(Debug, Component)]
#[relationship_target(relationship=VerticallyDraggedBy)]
pub struct VerticallyDrags(Vec<Entity>);

#[derive(Debug, Component)]
#[relationship(relationship_target=VerticallyDrags)]
pub struct VerticallyDraggedBy(pub Entity);

#[derive(Debug, Component)]
#[relationship_target(relationship=HorizontallyDraggedBy)]
pub struct HorizontallyDrags(Vec<Entity>);

#[derive(Debug, Component)]
#[relationship(relationship_target=HorizontallyDrags)]
pub struct HorizontallyDraggedBy(pub Entity);

#[derive(Debug, Bundle)]
pub struct DraggedBy(VerticallyDraggedBy, HorizontallyDraggedBy);

impl DraggedBy {
    pub fn new(entity: Entity) -> Self {
        Self(VerticallyDraggedBy(entity), HorizontallyDraggedBy(entity))
    }
}
