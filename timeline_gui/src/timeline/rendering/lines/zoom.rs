use bevy::{
    camera::visibility::RenderLayers, ecs::query::QueryEntityError, prelude::*,
    window::PrimaryWindow,
};
use timeline_core::date_iteration::YearIterator;

use crate::{
    timeline::rendering::{
        configuration::{
            TimelineHorizontalOffset, TimelineLineSeparation, TimelineRenderRange,
            TimelineScreenSize,
        },
        lines::VerticalLineRenderInfo,
    },
    zooming::{ZoomLevel, ZoomMessage},
};

pub fn update_offset_on_zoom(
    mut zoom_messages: MessageReader<ZoomMessage>,
    mut offset_query: Query<&mut TimelineHorizontalOffset>,
) {
    for message in zoom_messages.read() {
        match offset_query.get_mut(message.entity()) {
            Ok(mut offset) => {
                **offset = message.anchor().x.lerp(**offset, message.factor());
            }
            Err(e) => {
                error!("Couldn't update timeline offset on zoom: {e}");
            }
        }
    }
}

#[tracing::instrument(skip_all)]
#[expect(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    reason = "Lines layouting is best effort"
)]
pub fn handle_lines_zoom(
    mut zoom_messages: MessageReader<ZoomMessage>,
    mut timeline_info: Query<(
        &ZoomLevel,
        &TimelineLineSeparation,
        &mut TimelineRenderRange,
        Option<&TimelineScreenSize>,
    )>,

    spawn_lines_info: Query<(&Transform, &VerticalLineRenderInfo, &RenderLayers)>,
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    for message in zoom_messages.read() {
        let (&zoom_level, &line_separation, mut render_range, size) = match timeline_info
            .get_mut(message.entity())
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
        let render_size = size.map_or(window.size(), |s| **s);
        let occupied_space = (render_range.0.len() + 1) as f32 * scaled_line_separation;

        // TODO: Make aware of the proportions to each side.
        #[expect(
            clippy::cast_sign_loss,
            reason = "we use max 0 to ignore negative numbers, in case we don't need to add lines"
        )]
        let amount_to_extend = ((render_size.x - occupied_space) / (2. * scaled_line_separation))
            .max(0.)
            .ceil() as usize;
        if amount_to_extend == 0 {
            continue;
        }

        let right_iter = YearIterator::new(&render_range.0.end)
            .skip(1)
            .take(amount_to_extend);
        render_range.0.end = right_iter.clone().last().unwrap();
        super::TimelineLinesPlugin::spawn_vertical_lines(
            &mut commands,
            message.entity(),
            spawn_lines_info,
            right_iter.enumerate().map(move |(i, y)| {
                (
                    scaled_line_separation.mul_add((i + 1) as f32, occupied_space / 2.),
                    y,
                )
            }),
        )
        .unwrap();

        let left_iter = YearIterator::new(&render_range.0.start)
            .rev()
            .skip(1)
            .take(amount_to_extend);
        render_range.0.start = left_iter.clone().last().unwrap();
        super::TimelineLinesPlugin::spawn_vertical_lines(
            &mut commands,
            message.entity(),
            spawn_lines_info,
            left_iter.enumerate().map(move |(i, y)| {
                (
                    (-scaled_line_separation).mul_add((i + 1) as f32, -occupied_space / 2.),
                    y,
                )
            }),
        )
        .unwrap();
    }
}
