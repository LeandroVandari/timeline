use bevy::{prelude::*, window::PrimaryWindow};
use tracing::instrument;

use super::TimelineRenderInformation;

#[instrument(skip_all)]
pub fn spawn_dragging_background(
    mut commands: Commands,
    render_info_query: Query<(&TimelineRenderInformation, &Transform)>,
    mut added_render_infos: MessageReader<super::TimelineRenderInformationCreatedMessage>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    for msg in added_render_infos.read() {
        let entity = msg.entity();
        trace!("Spawning dragging background for timeline {entity}");
        let (render_info, pos) = render_info_query
            .get(entity)
            .expect("Message should refer to an entity with proper components");
        let timeline_size = render_info.size.unwrap_or(window.size());

        let background_entity = commands
            .spawn((
                Sprite {
                    custom_size: Some(timeline_size),
                    color: if cfg!(feature = "debug") {
                        Color::srgba(0.5, 0., 0., 0.5)
                    } else {
                        Color::NONE
                    },
                    ..Default::default()
                },
                pos.with_translation(Vec3::new(0., 0., -100.)),
                Pickable::default(),
                render_info.layers.clone(),
            ))
            .observe(
                |trigger: On<Pointer<Drag>>| {
                    if matches!(trigger.button, PointerButton::Primary) {}
                },
            )
            .id();

        commands.entity(entity).add_child(background_entity);
    }
}
