use bevy::{camera::visibility::RenderLayers, prelude::*, window::PrimaryWindow};
#[cfg(target_os = "macos")]
use bevy::{
    input::gestures::PinchGesture,
    picking::{hover::PickingInteraction, pointer::PointerInteraction},
};
use tracing::instrument;

#[cfg(target_os = "macos")]
use crate::timeline::rendering::configuration::TimelineLineSeparation;
use crate::timeline::rendering::configuration::TimelineScreenSize;
use bevy_drag::DragMessage;
use bevy_zoom::ZoomMessage;

#[instrument(skip_all)]
pub fn spawn_timeline_background(
    mut commands: Commands,
    render_info_query: Query<(Option<&TimelineScreenSize>, &Transform, &RenderLayers)>,
    mut added_render_infos: PopulatedMessageReader<super::RenderedTimelineCreatedMessage>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    for msg in added_render_infos.read() {
        let entity = msg.entity();
        trace!("Spawning background for timeline {entity}");
        let (size, pos, render_layers) = render_info_query
            .get(entity)
            .expect("Message should refer to an entity with proper components");
        let render_size = size.map_or(window.size(), |s| **s);

        commands
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
                #[cfg(target_os = "macos")]
                InteractionBackground,
                ChildOf(entity),
            ))
            .observe(emit_timeline_drag_message)
            .observe(emit_timeline_zoom_message);
    }
}

fn emit_timeline_drag_message(
    trigger: On<Pointer<Drag>>,
    child_query: Query<&ChildOf>,
    mut writer: MessageWriter<DragMessage>,
) {
    if matches!(trigger.button, PointerButton::Primary) {
        let timeline_entity = child_query
            .get(trigger.entity)
            .expect("Background entity is always child of a RenderedTimeline entity.")
            .parent();
        writer.write(DragMessage::new(timeline_entity, trigger.delta));
    }
}

fn emit_timeline_zoom_message(
    trigger: On<Pointer<Scroll>>,
    mut writer: MessageWriter<ZoomMessage>,
    child_query: Query<&ChildOf>,
) {
    if trigger.y == 0. {
        return;
    }

    let timeline_entity = child_query
        .get(trigger.entity)
        .expect("Background entity is always child of a RenderedTimeline entity.")
        .parent();
    let zoom_factor = 1. + trigger.y / 100.;

    writer.write(ZoomMessage::new(
        timeline_entity,
        zoom_factor,
        trigger.hit.position.unwrap().xy(),
    ));
}

#[cfg(target_os = "macos")]
#[derive(Debug, Component)]
pub struct InteractionBackground;

#[cfg(target_os = "macos")]
pub fn emit_timeline_zoom_message_on_pinch(
    mut pinch_messages: PopulatedMessageReader<PinchGesture>,

    mut writer: MessageWriter<ZoomMessage>,
    child_query: Query<&ChildOf>,
    mut line_separation_query: Query<&mut TimelineLineSeparation>,

    interaction_background: Query<(Entity, &PickingInteraction), With<InteractionBackground>>,
    pointer_interaction: Query<&PointerInteraction>,
) {
    let bg_entities: Vec<_> = interaction_background
        .iter()
        .filter(|(_, pick_state)| matches!(pick_state, PickingInteraction::Hovered))
        .map(|(entity, _)| entity)
        .collect();

    for &PinchGesture(zoom_factor) in pinch_messages.read() {
        let zoom_factor = 1. + zoom_factor;
        for bg in bg_entities.iter() {
            let timeline_entity = child_query
                .get(*bg)
                .expect("Background always is always child of a RenderedTimeline.")
                .parent();
            let pos = pointer_interaction
                .iter()
                .find_map(|interaction| {
                    interaction.iter().find_map(|(e, hit)| {
                        if e != bg {
                            None
                        } else {
                            Some(hit.position.unwrap().xy())
                        }
                    })
                })
                .unwrap();

            line_separation_query
                .get_mut(timeline_entity)
                .expect("RenderedTimeline always has a TimelineLineSeparation child.")
                .0 *= zoom_factor;

            writer.write(ZoomMessage::new(timeline_entity, zoom_factor, pos));
        }
    }
}
