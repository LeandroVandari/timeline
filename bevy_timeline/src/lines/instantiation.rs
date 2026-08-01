use bevy::{
    camera::visibility::RenderLayers, ecs::query::QueryEntityError, prelude::*,
    window::PrimaryWindow,
};
use bevy_zoom::{ZoomLevel, ZoomMessage};
use timeline_core::date_iteration::YearIterator;

use crate::{
    RenderedTimeline,
    configuration::{
        TimelineHorizontalOffset, TimelineLineSeparation, TimelineRenderRange, TimelineScreenSize,
        TimelineVerticalOffset,
    },
    lines::VerticalLineRenderInfo,
};

pub struct LineInstantiationPlugin;

impl Plugin for LineInstantiationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            Self::create_lines_on_zoom.in_set(LineInstantiationSet),
        );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct LineInstantiationSet;

impl LineInstantiationPlugin {
    #[expect(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        reason = "Lines layouting is best effort"
    )]
    fn create_lines_on_zoom(
        mut zoom_messages: PopulatedMessageReader<ZoomMessage>,
        mut timeline_info: Query<(
            &ZoomLevel,
            &TimelineLineSeparation,
            &mut TimelineRenderRange,
            &mut TimelineHorizontalOffset,
            Option<&TimelineScreenSize>,
        )>,

        spawn_lines_info: Query<(
            &Transform,
            &VerticalLineRenderInfo,
            &RenderLayers,
            &TimelineVerticalOffset,
            &ZoomLevel,
        )>,
        mut commands: Commands,
        window: Single<&Window, With<PrimaryWindow>>,
    ) {
        for message in zoom_messages.read() {
            let (&zoom_level, &line_separation, mut render_range, mut offset, size) =
                match timeline_info.get_mut(message.entity()) {
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
            let occupied_space =
                RenderedTimeline::draw_width(&render_range, line_separation, zoom_level);

            #[expect(
                clippy::cast_sign_loss,
                reason = "We explicitly set the min to 0 first"
            )]
            let amount_to_extend_right = ((render_size.x / 2. - (**offset + occupied_space / 2.))
                / scaled_line_separation)
                .max(0.)
                .ceil() as usize;
            if amount_to_extend_right != 0 {
                let right_iter = YearIterator::new(&render_range.0.end)
                    .skip(1)
                    .take(amount_to_extend_right);
                render_range.0.end = right_iter.clone().last().unwrap();
                let offset = **offset;
                super::TimelineLinesPlugin::spawn_vertical_lines(
                    &mut commands,
                    message.entity(),
                    spawn_lines_info,
                    right_iter.enumerate().map(move |(i, y)| {
                        (
                            scaled_line_separation
                                .mul_add((i + 1) as f32, occupied_space / 2. + offset),
                            y,
                        )
                    }),
                )
                .unwrap();
            }

            #[expect(
                clippy::cast_sign_loss,
                reason = "We explicitly set the min to 0 first"
            )]
            let amount_to_extend_left = (((**offset - occupied_space / 2.) + render_size.x / 2.)
                / scaled_line_separation)
                .max(0.)
                .ceil() as usize;
            if amount_to_extend_left != 0 {
                let left_iter = YearIterator::new(&render_range.0.start)
                    .rev()
                    .skip(1)
                    .take(amount_to_extend_left);
                render_range.0.start = left_iter.clone().last().unwrap();
                let offset = **offset;
                super::TimelineLinesPlugin::spawn_vertical_lines(
                    &mut commands,
                    message.entity(),
                    spawn_lines_info,
                    left_iter.enumerate().map(move |(i, y)| {
                        (
                            (-scaled_line_separation)
                                .mul_add((i + 1) as f32, -occupied_space / 2. + offset),
                            y,
                        )
                    }),
                )
                .unwrap();
            }
            #[expect(
                clippy::cast_possible_wrap,
                reason = "Will only wrap around on absurd quantities of lines"
            )]
            {
                **offset += (amount_to_extend_right as i64 - amount_to_extend_left as i64) as f32
                    * scaled_line_separation
                    / 2.;
            }
        }
    }
}
