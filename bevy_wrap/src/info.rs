use bevy::prelude::*;

#[derive(Debug, Component, Clone)]
pub struct WrapAroundInfo {
    pub center: f32,
    pub half_width: f32,
    pub emit_message: bool,
}

impl Default for WrapAroundInfo {
    fn default() -> Self {
        warn!(
            "Creating default WrapAroundInfo. \
            This is probably not intentional, as this will not wrap any entities around. \
            Spawn the component manually with the correct information."
        );
        Self {
            center: 0.,
            half_width: f32::MAX,
            emit_message: false,
        }
    }
}
