//! Handle vertical lines with year labels for [`RenderedTimeline`](crate::RenderedTimeline)s.
//!
//! Whenever a new [`RenderedTimeline`](crate::RenderedTimeline) is added as a component, this [`Plugin`] will add, based on the
//! [`configuration`](crate::configuration), the requested vertical lines with respective year labels.
//!
//! Additionally, it updates the lines whenever the timeline is [`zoom`](bevy_zoom::ZoomMessage)ed and [`drag`](bevy_drag::DragMessage)ged.
//! This update occurs conceptually in three fases:
//! 1. Translation - the lines are moved to their new positions based on the zoom and drag events. [Vertical](TimelineVerticalOffset) and [horizontal](crate::configuration::TimelineHorizontalOffset) offsets are updated accordingly.
//! 2. Instantiation - based on the new line positions, lines are added/removed as needed.
//! 3. Wrap around - Lines that are off-center wrap around to come at the other side of the screen.

use bevy::ecs::query::QueryEntityError;
use bevy::{camera::visibility::RenderLayers, prelude::*};
use timeline_core::date_iteration::year::Year;

use crate::configuration::TimelineVerticalOffset;
use crate::lines::relationship_label::LabelOf;

use crate::message::RenderedTimelineCreatedMessage;
use bevy_drag::{HorizontallyDraggedBy, VerticallyDraggedBy};
use bevy_wrap::WrapAround;
use bevy_zoom::ZoomLevel;

mod instantiation;
mod relationship_label;
mod setup;
mod translation;
mod wrapping;

mod window_resize;

pub struct TimelineLinesPlugin;

impl Plugin for TimelineLinesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            translation::LineTranslationPlugin,
            wrapping::LineWrapAroundPlugin,
            instantiation::LineInstantiationPlugin,
        ))
        .add_systems(
            Update,
            (
                Self::create_vertical_line_render_info,
                Self::spawn_new_timeline_lines,
            )
                .chain(),
        )
        .add_systems(Update, window_resize::update_line_meshes_on_window_resize)
        .configure_sets(
            Update,
            (
                translation::LineTranslationSet,
                instantiation::LineInstantiationSet,
                wrapping::LineWrapSet,
            )
                .chain(),
        );
    }
}

impl TimelineLinesPlugin {
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
}

#[derive(Debug, Component, Clone)]
struct VerticalLineRenderInfo {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

#[derive(Debug, Component)]
pub struct MainTimelineLine;

#[derive(Debug, Component)]
struct YearLabel(pub Year);
