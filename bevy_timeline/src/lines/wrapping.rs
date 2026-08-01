use core::cmp::Reverse;

use bevy::prelude::*;

use bevy_wrap::{WrapAround, WrapAroundInfo, WrapAroundMessage, WrapDirection};
use bevy_zoom::ZoomLevel;

use crate::{
    RenderedTimeline,
    configuration::{TimelineHorizontalOffset, TimelineLineSeparation, TimelineRenderRange},
    lines::{YearLabel, relationship_label::LabelOf},
};
pub struct LineWrapAroundPlugin;

impl Plugin for LineWrapAroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                Self::update_wrap_around_info.before(bevy_wrap::WrapAroundSet),
                (
                    Self::update_offset_on_wrap_around,
                    Self::update_year_label_on_wrap_around,
                )
                    .after(bevy_wrap::WrapAroundSet),
            )
                .in_set(LineWrapSet),
        );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, SystemSet)]
pub struct LineWrapSet;

impl LineWrapAroundPlugin {
    fn update_wrap_around_info(
        mut updated_timelines: Query<(
            &ZoomLevel,
            &TimelineLineSeparation,
            &TimelineRenderRange,
            &mut WrapAroundInfo,
        )>,
    ) {
        for (&zoom, &line_separation, render_range, mut wrap_around_info) in
            updated_timelines.iter_mut()
        {
            let occupied_space = RenderedTimeline::draw_width(render_range, line_separation, zoom);
            wrap_around_info.half_width = f32::midpoint(occupied_space, *line_separation * *zoom);
        }
    }
    fn update_offset_on_wrap_around(
        mut wrap_around_messages: PopulatedMessageReader<WrapAroundMessage>,
        mut timeline_info_query: Query<(
            &mut TimelineHorizontalOffset,
            &TimelineLineSeparation,
            &ZoomLevel,
        )>,
        wrap_around_query: Query<&WrapAround>,
    ) {
        for WrapAroundMessage { entity, direction } in wrap_around_messages.read() {
            let Ok(WrapAround(timeline_entity)) = wrap_around_query.get(*entity) else {
                return;
            };

            let (mut offset, &line_separation, &zoom_level) =
                timeline_info_query.get_mut(*timeline_entity).unwrap();
            match direction {
                WrapDirection::Left => **offset = line_separation.mul_add(*zoom_level, **offset),
                WrapDirection::Right => {
                    **offset = (-*line_separation).mul_add(*zoom_level, **offset);
                }
            }
        }
    }

    fn update_year_label_on_wrap_around(
        mut wrap_around_messages: PopulatedMessageReader<WrapAroundMessage>,

        wrapped_query: Query<&Children, With<WrapAround>>,

        mut label_query: Query<(Entity, &mut YearLabel, &mut Text2d, &LabelOf)>,
        mut timeline_range_query: Query<&mut TimelineRenderRange>,
    ) {
        let wrapped_labels = wrap_around_messages
            .read()
            .filter_map(
                |WrapAroundMessage {
                     entity: wrapped_entity,
                     direction,
                 }| {
                    let labels = label_query
                        .iter_many(wrapped_query.get(*wrapped_entity).ok()?)
                        .map(move |(entity, label, ..)| (entity, label.0, direction));

                    Some(labels)
                },
            )
            .flatten();

        let mut left = Vec::new();
        let mut right = Vec::new();

        wrapped_labels.for_each(|(entity, year, direction)| match direction {
            WrapDirection::Left => {
                left.push((entity, year));
            }
            WrapDirection::Right => right.push((entity, year)),
        });

        left.sort_unstable_by_key(|(_entity, year)| *year);
        right.sort_unstable_by_key(|(_entity, year)| Reverse(*year));

        for (v, dir) in [(left, WrapDirection::Left), (right, WrapDirection::Right)] {
            for (entity, _) in v {
                let (_entity, mut year, mut text_label, timeline_entity) = label_query
                    .get_mut(entity)
                    .expect("Just got the entity frow the same query");
                let mut range = timeline_range_query
                    .get_mut(timeline_entity.0)
                    .expect("Label refers to a `RenderedTimeline`");

                // Update the timeline range and the label
                match dir {
                    WrapDirection::Left => {
                        range.inc();
                        year.0 = range.0.end;
                    }
                    WrapDirection::Right => {
                        range.dec();
                        year.0 = range.0.start;
                    }
                }
                text_label.0 = year.0.to_string();
            }
        }
    }
}
