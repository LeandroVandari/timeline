use bevy::prelude::*;

#[derive(Debug, Message)]
pub struct ZoomMessage {
    zoomed_entity: Entity,
    factor: f32,
    anchor: Vec2,
}

impl ZoomMessage {
    pub fn new(zoomed_entity: Entity, factor: f32, position: Vec2) -> Self {
        Self {
            zoomed_entity,
            factor,
            anchor: position,
        }
    }

    pub fn entity(&self) -> Entity {
        self.zoomed_entity
    }

    pub fn factor(&self) -> f32 {
        self.factor
    }

    pub fn anchor(&self) -> Vec2 {
        self.anchor
    }
}
