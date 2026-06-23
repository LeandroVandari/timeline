use bevy::ecs::query::QueryEntityError;
use bevy::{camera::visibility::RenderLayers, prelude::*};
use timeline_core::date_iteration::year::Year;
use tracing::instrument;

use crate::timeline::rendering::configuration::TimelineRenderRange;
use crate::zooming::{ZoomMessage, ZoomSet};
use crate::{
    dragging::{
        HorizontalWrapAround, WrapAround, WrapDirection,
        relationship::{DraggedBy, HorizontallyDraggedBy},
    },
    timeline::rendering::configuration::RenderedTimelineCreatedMessage,
};

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
                .chain()
                .run_if(on_message::<RenderedTimelineCreatedMessage>),
        )
        .add_systems(
            Update,
            zoom::handle_lines_zoom
                .run_if(on_message::<ZoomMessage>)
                .after(ZoomSet),
        )
        .add_observer(Self::year_label_wrap_around);
    }
}

impl TimelineLinesPlugin {
    #[instrument(skip_all)]
    fn spawn_vertical_lines(
        commands: &mut Commands,
        timeline_entity: Entity,

        timeline_info: Query<(&Transform, &VerticalLineRenderInfo, &RenderLayers)>,
        lines: impl Iterator<Item = (f32, Year)> + Send + Sync + 'static + Clone,
    ) -> Result<(), QueryEntityError> {
        trace!("Spawning vertical lines for timeline {timeline_entity}");
        let (pos, render_info, render_layers) = timeline_info.get(timeline_entity)?;

        {
            let pos = *pos;
            let render_info = render_info.to_owned();
            let render_layers = render_layers.to_owned();
            let lines = lines.clone().map(move |(line_x_pos, _year)| {
                (
                    render_info.wrap_info,
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
            let render_info = render_info.to_owned();
            let render_layers = render_layers.to_owned();

            let labels = lines.map(move |(line_x_pos, year)| {
                (
                    render_info.wrap_info,
                    DraggedBy::new(timeline_entity),
                    Text2d::new(year.to_string()),
                    YearLabel(year),
                    pos.with_translation(Vec3::new(line_x_pos, -15., 0.)),
                    render_layers.clone(),
                    ChildOf(timeline_entity),
                )
            });
            commands.spawn_batch(labels);
        }

        Ok(())
    }

    fn year_label_wrap_around(
        trigger: On<WrapAround>,
        mut label_query: Query<(&mut YearLabel, &mut Text2d, &ChildOf)>,
        mut range_query: Query<&mut TimelineRenderRange>,
    ) {
        let Ok((mut year, mut text_label, parent)) = label_query.get_mut(trigger.entity) else {
            return;
        };
        let mut range = range_query
            .get_mut(parent.0)
            .expect("`entity` is the RenderedTimeline which also has a TimelineRenderRange.");
        match trigger.direction {
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

#[derive(Debug, Component, Clone)]
struct VerticalLineRenderInfo {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
    wrap_info: HorizontalWrapAround,
}

#[derive(Debug, Component)]
struct YearLabel(pub Year);
