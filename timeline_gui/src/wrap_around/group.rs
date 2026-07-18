use bevy::prelude::*;

use crate::wrap_around::WrapAroundInfo;

#[derive(Debug, Component)]
#[relationship_target(relationship = super::WrapAround)]
#[require(WrapAroundInfo)]
pub struct WrapAroundGroup(Vec<Entity>);
