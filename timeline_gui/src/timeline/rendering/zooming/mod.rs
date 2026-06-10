use bevy::{input::gestures::PinchGesture, prelude::*};

mod message;

pub use message::ZoomMessage;
use tracing::instrument;

use crate::{
    query_ext::QueryExt as _,
    timeline::rendering::{
        background,
        dragging::relationship::{
            HorizontallyDraggedBy, HorizontallyDrags, VerticallyDraggedBy, VerticallyDrags,
        },
    },
};

pub struct ZoomingPlugin;

impl Plugin for ZoomingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::handle_zoom.run_if(on_message::<ZoomMessage>))
            .add_message::<ZoomMessage>();

        #[cfg(target_os = "macos")]
        app.add_systems(
            Update,
            background::emit_timeline_zoom_message_on_pinch.run_if(on_message::<PinchGesture>),
        );
    }
}

impl ZoomingPlugin {
    #[expect(clippy::type_complexity, reason = "Bevy Queries are 'complex types'")]
    #[instrument(skip_all)]
    fn handle_zoom(
        mut zoom_messages: MessageReader<ZoomMessage>,

        mut zoom_query: Query<
            &mut Transform,
            Or<(With<VerticallyDraggedBy>, With<HorizontallyDraggedBy>)>,
        >,

        vertically_drags_query: Query<&VerticallyDrags>,
        horizontally_drags_query: Query<&HorizontallyDrags>,
    ) {
        for message in zoom_messages.read() {
            let anchor = message.anchor();
            let zoomed_entity = message.entity();

            horizontally_drags_query.for_each_matching(
                zoomed_entity,
                &mut zoom_query,
                |mut pos| {
                    pos.translation.x =
                        (pos.translation.x - anchor.x).mul_add(message.factor(), anchor.x);
                },
            );

            vertically_drags_query.for_each_matching(zoomed_entity, &mut zoom_query, |mut pos| {
                pos.translation.y =
                    (pos.translation.y - anchor.y).mul_add(message.factor(), anchor.y);
            });
        }
    }
}
