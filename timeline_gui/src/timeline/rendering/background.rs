use bevy::{camera::visibility::RenderLayers, prelude::*, window::PrimaryWindow};
use tracing::instrument;

use crate::timeline::rendering::{configuration::TimelineSize, dragging::DragMessage};

#[instrument(skip_all)]
pub fn spawn_timeline_background(
    mut commands: Commands,
    render_info_query: Query<(Option<&TimelineSize>, &Transform, &RenderLayers)>,
    mut added_render_infos: MessageReader<super::RenderedTimelineCreatedMessage>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    for msg in added_render_infos.read() {
        let entity = msg.entity();
        trace!("Spawning background for timeline {entity}");
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
            .observe(emit_timeline_drag_message)
            .id();

        commands.entity(entity).add_child(background_entity);
    }
}

fn emit_timeline_drag_message(
    trigger: On<Pointer<Drag>>,
    child_query: Query<&ChildOf>,
    mut writer: MessageWriter<DragMessage>,
) {
    if matches!(trigger.button, PointerButton::Primary) {
        let timeline_entity = child_query.get(trigger.entity).unwrap().parent();
        writer.write(DragMessage::new(timeline_entity, trigger.delta));
    }
}
