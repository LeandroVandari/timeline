use bevy::prelude::*;

#[derive(Debug, Message)]
pub struct WrapAroundMessage {
    pub entity: Entity,
    pub direction: super::WrapDirection,
}
