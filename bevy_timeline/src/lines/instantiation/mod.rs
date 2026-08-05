use bevy::prelude::*;

use line_deficit::LineDeficit;

mod create;
mod line_deficit;

pub struct LineInstantiationPlugin;

impl Plugin for LineInstantiationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (line_deficit::calculate_line_deficit, create::create_lines)
                .chain()
                .in_set(LineInstantiationSet),
        )
        .add_message::<LineDeficit>();
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct LineInstantiationSet;
