use bevy::{camera::visibility::RenderLayers, ecs::query::QueryEntityError, prelude::*};
use bevy_zoom::ZoomLevel;
use timeline_core::date_iteration::YearIterator;

use super::line_deficit::LineDeficit;
use crate::{
    RenderedTimeline,
    configuration::{
        TimelineHorizontalOffset, TimelineLineSeparation, TimelineRenderRange,
        TimelineVerticalOffset,
    },
    lines::VerticalLineRenderInfo,
};

#[expect(clippy::cast_precision_loss, reason = "Lines layouting is best effort")]
pub fn create_lines(
    mut deficit_reader: PopulatedMessageReader<LineDeficit>,
    mut timeline_info: Query<(
        &ZoomLevel,
        &TimelineLineSeparation,
        &mut TimelineRenderRange,
        &TimelineHorizontalOffset,
    )>,

    spawn_lines_info: Query<(
        &Transform,
        &VerticalLineRenderInfo,
        &RenderLayers,
        &TimelineVerticalOffset,
        &ZoomLevel,
    )>,
    mut commands: Commands,
) {
    for deficit in deficit_reader.read() {
        let (&zoom_level, &line_separation, mut render_range, &offset) = match timeline_info
            .get_mut(deficit.timeline)
        {
            Ok(tup) => tup,
            Err(QueryEntityError::QueryDoesNotMatch(_, _)) => {
                warn!(
                    "Timeline didn't contain required components for handling zoom of vertical lines"
                );
                continue;
            }
            Err(e) => {
                error!("Error matching query while handling zoom for timeline lines: {e}");
                continue;
            }
        };

        let scaled_line_separation = *line_separation * *zoom_level;
        let occupied_space =
            RenderedTimeline::draw_width(&render_range, line_separation, zoom_level);

        if deficit.right > 0 {
            let right_iter = YearIterator::new(&render_range.0.end)
                .skip(1)
                .take(deficit.right.cast_unsigned());
            render_range.0.end = right_iter.clone().last().unwrap();

            crate::lines::TimelineLinesPlugin::spawn_vertical_lines(
                &mut commands,
                deficit.timeline,
                spawn_lines_info,
                right_iter.enumerate().map(move |(i, y)| {
                    (
                        scaled_line_separation
                            .mul_add((i + 1) as f32, occupied_space / 2. + *offset),
                        y,
                    )
                }),
            )
            .unwrap();
        }

        if deficit.left > 0 {
            let left_iter = YearIterator::new(&render_range.0.start)
                .rev()
                .skip(1)
                .take(deficit.left.cast_unsigned());
            render_range.0.start = left_iter.clone().last().unwrap();

            crate::lines::TimelineLinesPlugin::spawn_vertical_lines(
                &mut commands,
                deficit.timeline,
                spawn_lines_info,
                left_iter.enumerate().map(move |(i, y)| {
                    (
                        (-scaled_line_separation)
                            .mul_add((i + 1) as f32, -occupied_space / 2. + *offset),
                        y,
                    )
                }),
            )
            .unwrap();
        }
    }
}
