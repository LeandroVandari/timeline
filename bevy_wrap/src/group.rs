use bevy::prelude::*;

use super::WrapAroundInfo;

#[derive(Debug, Component)]
#[relationship_target(relationship = super::WrapAround)]
#[require(WrapAroundInfo)]
pub struct WrapAroundGroup(Vec<Entity>);
