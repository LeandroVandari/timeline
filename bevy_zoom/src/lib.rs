use bevy::prelude::*;

mod message;
mod system_set;

pub use message::ZoomMessage;
pub use system_set::ZoomSet;

use bevy_drag::{HorizontallyDraggedBy, HorizontallyDrags, VerticallyDraggedBy, VerticallyDrags};
use query_ext::QueryExt as _;

pub struct ZoomingPlugin;

impl Plugin for ZoomingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::handle_zoom.in_set(ZoomSet))
            .add_message::<ZoomMessage>();
    }
}

#[derive(Debug, Component, Clone, Copy, Deref)]
pub struct ZoomLevel(f32);

impl ZoomingPlugin {
    #[expect(clippy::type_complexity, reason = "Bevy Queries are 'complex types'")]
    fn handle_zoom(
        mut zoom_messages: PopulatedMessageReader<ZoomMessage>,

        mut zoom_query: Query<
            &mut Transform,
            Or<(With<VerticallyDraggedBy>, With<HorizontallyDraggedBy>)>,
        >,

        mut zoom_level: Query<&mut ZoomLevel>,

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
                    pos.translation.x = anchor.x.lerp(pos.translation.x, message.factor());
                },
            );

            vertically_drags_query.for_each_matching(zoomed_entity, &mut zoom_query, |mut pos| {
                pos.translation.y = anchor.y.lerp(pos.translation.y, message.factor());
            });

            zoom_level
                .get_mut(zoomed_entity)
                .expect("Entities that are zoomed must have a `ZoomLevel` component.")
                .0 *= message.factor();
        }
    }
}

impl Default for ZoomLevel {
    fn default() -> Self {
        Self(1.0)
    }
}
