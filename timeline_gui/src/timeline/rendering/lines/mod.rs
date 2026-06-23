use bevy::ecs::query::QueryEntityError;
use bevy::{camera::visibility::RenderLayers, prelude::*};
use timeline_core::date_iteration::year::Year;
use tracing::instrument;

use crate::timeline::rendering::configuration::TimelineRenderRange;
use crate::{
    dragging::{
        HorizontalWrapAround, WrapAround, WrapDirection,
        relationship::{DraggedBy, HorizontallyDraggedBy},
    },
    timeline::rendering::configuration::RenderedTimelineCreatedMessage,
};

mod setup;

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
        );
    }
}

impl TimelineLinesPlugin {
    #[instrument(skip_all)]
    fn spawn_vertical_lines(
        commands: &mut Commands,
        timeline_entity: Entity,

        timeline_info: Query<(&Transform, &VerticalLineRenderInfo, &RenderLayers)>,
        lines: impl Iterator<Item = (f32, Year)>,
    ) -> Result<(), QueryEntityError> {
        trace!("Spawning vertical lines for timeline {timeline_entity}");
        let (pos, render_info, render_layers) = timeline_info.get(timeline_entity)?;

        commands.entity(timeline_entity).with_children(|spawner|{
            for (line_x_pos, year) in lines {
            spawner.spawn((
                    render_info.wrap_info,
                    HorizontallyDraggedBy(timeline_entity),
                    Mesh2d(render_info.mesh.clone()),
                    MeshMaterial2d(render_info.material.clone()),
                    pos.with_translation(Vec3::new(line_x_pos, 0., 0.)),
                    render_layers.clone(),
                ));
                spawner
                    .spawn((
                        render_info.wrap_info,
                        DraggedBy::new(timeline_entity),
                        Text2d::new(year.to_string()),
                        YearLabel(year),
                        pos.with_translation(Vec3::new(line_x_pos, -15., 0.)),
                        render_layers.clone(),
                    ))
                    .observe(
                        move |trigger: On<WrapAround>,
                              mut label_query: Query<(&mut YearLabel, &mut Text2d)>,
                              mut range_query: Query<&mut TimelineRenderRange>| {
                            let (mut year, mut text_label) =
                                label_query.get_mut(trigger.entity).expect("The entity has a Text2d and YearLabel");
                            let mut range = range_query.get_mut(timeline_entity).expect("`entity` is the RenderedTimeline which also has a TimelineRenderRange.");
                            match trigger.direction {
                                WrapDirection::Left => {
                                    range.inc();
                                    year.0 = range.0.end.clone();},
                                WrapDirection::Right => {
                                    range.dec();
                                    year.0 = range.0.start.clone();
                                }
                            }
                            text_label.0 = year.0.to_string();
                        },
        );}});

        Ok(())
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
