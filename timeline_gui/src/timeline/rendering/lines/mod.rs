use core::cmp::Reverse;

use bevy::ecs::query::QueryEntityError;
use bevy::{camera::visibility::RenderLayers, prelude::*};
use timeline_core::date_iteration::year::Year;
use tracing::instrument;

use crate::timeline::rendering::configuration::RenderedTimelineCreatedMessage;
use crate::timeline::rendering::configuration::{
    TimelineHorizontalOffset, TimelineLineSeparation, TimelineRenderRange, TimelineVerticalOffset,
};
use crate::timeline::rendering::lines::relationship_label::LabelOf;
use bevy_drag::relationship::{HorizontallyDraggedBy, VerticallyDraggedBy};
use bevy_wrap::{WrapAround, WrapAroundMessage, WrapAroundSet, WrapDirection};
use bevy_zoom::{ZoomLevel, ZoomMessage, ZoomSet};

mod drag;
mod setup;
mod zoom;

mod relationship_label;

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
                    .before(WrapAroundSet)
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
                .after(WrapAroundSet),
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

        let lines = {
            let pos = *pos;
            let render_info = render_info.to_owned();
            let render_layers = render_layers.to_owned();
            let vertical_offset = vertical_offset.to_owned();
            lines.clone().map(move |(line_x_pos, year)| {
                (
                    WrapAround(timeline_entity),
                    HorizontallyDraggedBy(timeline_entity),
                    Mesh2d(render_info.mesh.clone()),
                    MeshMaterial2d(render_info.material.clone()),
                    pos.with_translation(Vec3::new(line_x_pos, 0., 0.)),
                    render_layers.clone(),
                    ChildOf(timeline_entity),
                    children![(
                        VerticallyDraggedBy(timeline_entity),
                        Text2d::new(year.to_string()),
                        YearLabel(year),
                        pos.with_translation(Vec3::new(
                            0.,
                            15.0_f32.mul_add(-*zoom, *vertical_offset),
                            0.,
                        )),
                        render_layers.clone(),
                        LabelOf(timeline_entity),
                    )],
                )
            })
        };
        lines.for_each(|line_bundle| {
            commands.spawn(line_bundle);
        });

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

    #[tracing::instrument]
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

#[derive(Debug, Component, Clone)]
struct VerticalLineRenderInfo {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

#[derive(Debug, Component)]
struct YearLabel(pub Year);
