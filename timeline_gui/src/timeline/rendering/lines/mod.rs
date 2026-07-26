use bevy::ecs::query::QueryEntityError;
use bevy::{camera::visibility::RenderLayers, prelude::*};
use timeline_core::date_iteration::year::Year;
use tracing::instrument;

use crate::timeline::rendering::configuration::{
    TimelineHorizontalOffset, TimelineLineSeparation, TimelineRenderRange, TimelineVerticalOffset,
};
use crate::wrap_around::{self, WrapAround, WrapAroundMessage, WrapDirection};
use crate::zooming::{ZoomLevel, ZoomMessage, ZoomSet};
use crate::{
    dragging::relationship::{DraggedBy, HorizontallyDraggedBy},
    timeline::rendering::configuration::RenderedTimelineCreatedMessage,
};

mod drag;
mod setup;
mod zoom;

pub struct TimelineLinesPlugin;

impl Plugin for TimelineLinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                Self::create_vertical_line_render_info,
                Self::spawn_timeline_lines,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                zoom::update_offset_on_zoom,
                zoom::create_lines_on_zoom,
                zoom::update_wrap_around_info_on_zoom
                    .before(wrap_around::WrapAroundSet)
                    .run_if(on_message::<ZoomMessage>),
            )
                .chain()
                .after(ZoomSet),
        )
        .add_systems(Update, drag::update_timeline_offset_on_drag)
        .add_systems(
            Update,
            (
                Self::update_year_label_on_wrap_around,
                Self::update_timeline_offset_on_wrap_around,
            )
                .after(wrap_around::WrapAroundSet),
        );
    }
}

impl TimelineLinesPlugin {
    #[instrument(skip_all)]
    fn spawn_vertical_lines(
        commands: &mut Commands,
        timeline_entity: Entity,

        timeline_info: Query<(
            &Transform,
            &VerticalLineRenderInfo,
            &RenderLayers,
            &TimelineVerticalOffset,
            &ZoomLevel,
        )>,
        lines: impl Iterator<Item = (f32, Year)> + Send + Sync + 'static + Clone,
    ) -> Result<(), QueryEntityError> {
        trace!("Spawning vertical lines for timeline {timeline_entity}");
        let (pos, render_info, render_layers, vertical_offset, &zoom) =
            timeline_info.get(timeline_entity)?;

        {
            let pos = *pos;
            let render_info = render_info.to_owned();
            let render_layers = render_layers.to_owned();
            let lines = lines.clone().map(move |(line_x_pos, _year)| {
                (
                    WrapAround(timeline_entity),
                    HorizontallyDraggedBy(timeline_entity),
                    Mesh2d(render_info.mesh.clone()),
                    MeshMaterial2d(render_info.material.clone()),
                    pos.with_translation(Vec3::new(line_x_pos, 0., 0.)),
                    render_layers.clone(),
                    ChildOf(timeline_entity),
                )
            });

            commands.spawn_batch(lines);
        }

        {
            let pos = *pos;
            let render_layers = render_layers.to_owned();
            let vertical_offset = vertical_offset.to_owned();
            let labels = lines.map(move |(line_x_pos, year)| {
                (
                    WrapAround(timeline_entity),
                    DraggedBy::new(timeline_entity),
                    Text2d::new(year.to_string()),
                    YearLabel(year),
                    pos.with_translation(Vec3::new(
                        line_x_pos,
                        15.0_f32.mul_add(-*zoom, *vertical_offset),
                        0.,
                    )),
                    render_layers.clone(),
                    ChildOf(timeline_entity),
                )
            });
            commands.spawn_batch(labels);
        }

        Ok(())
    }

    fn update_timeline_offset_on_wrap_around(
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
        mut label_query: Query<(&mut YearLabel, &mut Text2d, &ChildOf)>,
        mut timeline_range_query: Query<&mut TimelineRenderRange>,
    ) {
        for WrapAroundMessage { entity, direction } in wrap_around_messages.read() {
            let Ok((mut year, mut text_label, parent)) = label_query.get_mut(*entity) else {
                return;
            };
            let mut range = timeline_range_query
            .get_mut(parent.0)
            .expect("`entity` is the RenderedTimeline which also has a TimelineRenderRange and TimelineHorizontalPosition.");

            // Update the timeline range and the label
            match direction {
                WrapDirection::Left => {
                    range.inc();
                    year.0 = range.0.end.clone();
                }
                WrapDirection::Right => {
                    range.dec();
                    year.0 = range.0.start.clone();
                }
            }
            text_label.0 = year.0.to_string();
        }
    }
}

#[derive(Debug, Component, Clone)]
struct VerticalLineRenderInfo {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

#[derive(Debug, Component)]
struct YearLabel(pub Year);
