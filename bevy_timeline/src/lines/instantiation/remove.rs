use bevy::{platform::collections::HashSet, prelude::*};
use timeline_core::date_iteration::YearIterator;

use crate::{
    configuration::TimelineRenderRange,
    lines::{YearLabel, instantiation::line_deficit::LineDeficit, relationship_label::Labels},
};

pub fn remove_extra_lines(
    mut commands: Commands,
    mut deficit_reader: PopulatedMessageReader<LineDeficit>,
    labels_query: Query<(&YearLabel, &ChildOf)>,
    mut timeline_query: Query<(&mut TimelineRenderRange, &Labels)>,
) {
    for deficit in deficit_reader.read() {
        if deficit.left >= 0 && deficit.right >= 0 {
            continue;
        }

        let Ok((mut render_range, timeline_labels)) = timeline_query.get_mut(deficit.timeline)
        else {
            warn!("Timeline without required components for removal...");
            continue;
        };

        let remove_right = (-deficit.right).max(0).cast_unsigned();
        let remove_left = (-deficit.left).max(0).cast_unsigned();

        let remove_right = YearIterator::new(&render_range.0.end)
            .rev()
            .take(remove_right);
        let remove_left = YearIterator::new(&render_range.0.start).take(remove_left);

        if let Some(last_removed) = remove_right.clone().last() {
            render_range.0.end = last_removed.get_previous().unwrap();
        }
        if let Some(last_removed) = remove_left.clone().last() {
            render_range.0.start = last_removed.get_next().unwrap();
        }

        let to_remove: HashSet<_> = remove_left.chain(remove_right).collect();

        labels_query
            .iter_many(timeline_labels.iter())
            .filter(|(year, _)| to_remove.contains(&year.0))
            .take(to_remove.len())
            .for_each(|(_, &ChildOf(line_entity))| {
                commands.entity(line_entity).despawn();
            });
    }
}
