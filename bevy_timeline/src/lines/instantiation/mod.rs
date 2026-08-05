use bevy::prelude::*;

use bevy_zoom::ZoomLevel;
use line_deficit::LineDeficit;

use crate::configuration::{TimelineHorizontalOffset, TimelineLineSeparation};

mod create;
mod line_deficit;

pub struct LineInstantiationPlugin;

impl Plugin for LineInstantiationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                line_deficit::calculate_line_deficit,
                create::create_lines,
                Self::update_timeline_offset,
            )
                .chain()
                .in_set(LineInstantiationSet),
        )
        .add_message::<LineDeficit>();
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct LineInstantiationSet;

impl LineInstantiationPlugin {
    #[expect(
        clippy::cast_precision_loss,
        reason = "Should only be a problem for huge deficits, which shouldn't happen"
    )]
    fn update_timeline_offset(
        mut deficit_reader: MessageReader<LineDeficit>,
        mut timeline_query: Query<(
            &mut TimelineHorizontalOffset,
            &ZoomLevel,
            &TimelineLineSeparation,
        )>,
    ) {
        for deficit in deficit_reader.read() {
            let (mut offset, &zoom_level, &line_separation) = timeline_query
                .get_mut(deficit.timeline)
                .expect("Deficit refers to a RenderedTimeline");

            let scaled_line_separation = *zoom_level * *line_separation;

            **offset +=
                (deficit.right.max(0) - deficit.left.max(0)) as f32 * scaled_line_separation / 2.;
        }
    }
}
