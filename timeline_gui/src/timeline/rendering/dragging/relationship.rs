use bevy::prelude::*;

#[derive(Debug, Component)]
#[relationship_target(relationship=DraggedBy)]
pub struct Drags(Vec<Entity>);

#[derive(Debug, Component)]
#[relationship(relationship_target=Drags)]
pub struct DraggedBy(pub Entity);
