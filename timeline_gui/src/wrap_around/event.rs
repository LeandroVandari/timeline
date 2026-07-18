use bevy::prelude::*;

#[derive(Debug, EntityEvent)]
pub struct WrapAroundEvent {
    pub entity: Entity,
    pub direction: super::WrapDirection,
}
