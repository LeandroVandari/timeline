use bevy::prelude::*;
use tracing::instrument;

pub use messages::DragMessage;
use query_ext::QueryExt as _;
pub use relationship::{
    DraggedBy, HorizontallyDraggedBy, HorizontallyDrags, VerticallyDraggedBy, VerticallyDrags,
};

mod messages;
mod relationship;

pub struct DraggingPlugin;

impl Plugin for DraggingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::handle_drag)
            .add_message::<DragMessage>();
    }
}

impl DraggingPlugin {
    #[expect(clippy::type_complexity, reason = "Bevy Queries are 'complex types'")]
    #[instrument(skip_all)]
    fn handle_drag(
        mut drag_messages: PopulatedMessageReader<DragMessage>,

        mut drag_query: Query<
            &mut Transform,
            Or<(With<VerticallyDraggedBy>, With<HorizontallyDraggedBy>)>,
        >,

        vertically_drags_query: Query<&VerticallyDrags>,
        horizontally_drags_query: Query<&HorizontallyDrags>,
    ) {
        for DragMessage { drag_entity, delta } in drag_messages.read() {
            vertically_drags_query.for_each_matching(*drag_entity, &mut drag_query, |mut pos| {
                pos.translation.y += delta.y;
            });

            horizontally_drags_query.for_each_matching(*drag_entity, &mut drag_query, |mut pos| {
                pos.translation.x += delta.x;
            });
        }
    }
}
