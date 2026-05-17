use bevy::{camera::visibility::RenderLayers, prelude::*, window::PrimaryWindow};
use tracing::instrument;

use crate::timeline::rendering::{
    configuration::TimelineSize,
    dragging::relationship::DraggedBy,
    lines::{MainLine, VerticalLine},
};

pub mod relationship;

#[instrument(skip_all)]
pub fn spawn_dragging_background(
    mut commands: Commands,
    render_info_query: Query<(Option<&TimelineSize>, &Transform, &RenderLayers)>,
    mut added_render_infos: MessageReader<super::RenderedTimelineCreatedMessage>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    for msg in added_render_infos.read() {
        let entity = msg.entity();
        trace!("Spawning dragging background for timeline {entity}");
        let (size, pos, render_layers) = render_info_query
            .get(entity)
            .expect("Message should refer to an entity with proper components");
        let render_size = size.map_or(window.size(), |s| **s);

        let background_entity = commands
            .spawn((
                Sprite {
                    custom_size: Some(render_size),
                    color: if cfg!(feature = "debug") {
                        Color::srgba(0.5, 0., 0., 0.5)
                    } else {
                        Color::NONE
                    },
                    ..Default::default()
                },
                pos.with_translation(Vec3::new(0., 0., -100.)),
                Pickable::default(),
                render_layers.clone(),
            ))
            .observe(handle_timeline_drag)
            .id();

        commands.entity(entity).add_child(background_entity);
    }
}

type VerticalLinesFilter = (With<VerticalLine>, Without<MainLine>, Without<DraggedBy>);
type MainLineFilter = (With<MainLine>, Without<VerticalLine>, Without<DraggedBy>);
type ToDragFilter = (Without<MainLine>, Without<VerticalLine>);

/// Handle translation of the timeline's labels/lines/rendered events when it's dragged with the mouse.
#[instrument(skip_all)]
fn handle_timeline_drag(
    trigger: On<Pointer<Drag>>,

    childof_query: Query<&ChildOf>,

    mut vertical_lines_query: Query<(&ChildOf, &mut Transform), VerticalLinesFilter>,
    mut main_line_query: Query<(&ChildOf, &mut Transform), MainLineFilter>,

    mut to_drag_query: Query<(&DraggedBy, &mut Transform), ToDragFilter>,
) {
    if matches!(trigger.button, PointerButton::Primary) {
        // Since we only want to move the timeline that was dragged, the whole thing needs to traverse the relationship tree, otherwise we will edit
        // all of the timelines on screen.
        let timeline_entity = childof_query.get(trigger.entity).unwrap().parent();

        vertical_lines_query
            .iter_mut()
            .filter(|(childof, _)| childof.parent() == timeline_entity)
            .for_each(|(_, mut pos)| pos.translation.x += trigger.delta.x);

        main_line_query
            .iter_mut()
            .filter(|(childof, _)| childof.parent() == timeline_entity)
            .for_each(|(_, mut pos)| pos.translation.y -= trigger.delta.y);

        to_drag_query
            .iter_mut()
            .filter(|(dragged_by, _)| dragged_by.0 == timeline_entity)
            .for_each(|(_, mut pos)| {
                pos.translation.x += trigger.delta.x;
                pos.translation.y -= trigger.delta.y;
            });
    }
}
