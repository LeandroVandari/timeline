use bevy::prelude::*;

#[derive(Debug, Component)]
#[relationship(relationship_target=Labels)]
pub struct LabelOf(pub Entity);

#[derive(Debug, Component)]
#[relationship_target(relationship=LabelOf)]
pub struct Labels(Vec<Entity>);
