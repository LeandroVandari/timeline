use bevy::{prelude::*, window::PrimaryWindow};
use bevy_zoom::ZoomLevel;

use crate::{
    RenderedTimeline,
    configuration::{
        TimelineHorizontalOffset, TimelineLineSeparation, TimelineRenderRange, TimelineScreenSize,
    },
};

#[derive(Debug, Message)]
pub struct LineDeficit {
    pub timeline: Entity,
    pub left: isize,
    pub right: isize,
}

#[expect(
    clippy::cast_possible_truncation,
    reason = "Deficit is calculated with f32 but is ceiled before converting to int, and shouldn't be so large that i32 can't handle it"
)]
#[expect(clippy::type_complexity, reason = "bevy queries are complex")]
pub fn calculate_line_deficit(
    mut deficit_writer: MessageWriter<LineDeficit>,
    timelines_query: Query<(
        Entity,
        &ZoomLevel,
        &TimelineLineSeparation,
        &TimelineRenderRange,
        &TimelineHorizontalOffset,
        Option<&TimelineScreenSize>,
    )>,

    window: Single<&Window, With<PrimaryWindow>>,
) {
    for (timeline_entity, &zoom_level, &line_separation, render_range, &offset, size) in
        timelines_query.iter()
    {
        let scaled_line_separation = *line_separation * *zoom_level;
        let render_size = size.map_or(window.size(), |s| **s);
        let occupied_space =
            RenderedTimeline::draw_width(render_range, line_separation, zoom_level);

        let deficit_right = ((render_size.x / 2. - (*offset + occupied_space / 2.))
            / scaled_line_separation)
            .ceil() as isize;
        let deficit_left = (((*offset - occupied_space / 2.) + render_size.x / 2.)
            / scaled_line_separation)
            .ceil() as isize;

        if deficit_right == 0 && deficit_left == 0 {
            continue;
        }
        debug!(
            "Line Deficit for timeline {timeline_entity}: left: {deficit_left}, right: {deficit_right}"
        );

        deficit_writer.write(LineDeficit {
            timeline: timeline_entity,
            left: deficit_left,
            right: deficit_right,
        });
    }
}
