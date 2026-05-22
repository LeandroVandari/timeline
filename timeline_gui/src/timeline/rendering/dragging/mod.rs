use bevy::{ecs::entity::EntityHashSet, prelude::*};
use tracing::instrument;

use crate::timeline::rendering::dragging::relationship::{
    HorizontallyDraggedBy, HorizontallyDrags, VerticallyDraggedBy, VerticallyDrags,
};
pub use messages::DragMessage;

mod messages;
pub mod relationship;

pub struct DraggingPlugin;

impl Plugin for DraggingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::handle_drag.run_if(on_message::<DragMessage>))
            .add_message::<DragMessage>();
    }
}

impl DraggingPlugin {
    #[expect(clippy::type_complexity, reason = "Bevy Queries are 'complex types'")]
    #[instrument(skip_all)]
    fn handle_drag(
        mut drag_messages: MessageReader<DragMessage>,

        mut drag_query: Query<
            &mut Transform,
            Or<(With<VerticallyDraggedBy>, With<HorizontallyDraggedBy>)>,
        >,

        vertically_drags_query: Query<&VerticallyDrags>,
        horizontally_drags_query: Query<&HorizontallyDrags>,
    ) {
        for &DragMessage {
            dragged_entity,
            delta,
        } in drag_messages.read()
        {
            match vertically_drags_query.get(dragged_entity) {
                Ok(dragged) => {
                    drag_query
                        .iter_many_unique_mut(EntityHashSet::from_iter(
                            dragged.collection().iter().copied(),
                        ))
                        .for_each(|mut pos| pos.translation.y -= delta.y);
                }
                Err(bevy::ecs::query::QueryEntityError::QueryDoesNotMatch(_, _)) => (),
                Err(e) => error!("Error running drag query: {e}"),
            }

            match horizontally_drags_query.get(dragged_entity) {
                Ok(dragged) => {
                    drag_query
                        .iter_many_unique_mut(EntityHashSet::from_iter(
                            dragged.collection().iter().copied(),
                        ))
                        .for_each(|mut pos| pos.translation.x += delta.x);
                }
                Err(bevy::ecs::query::QueryEntityError::QueryDoesNotMatch(_, _)) => (),
                Err(e) => error!("Error running drag query: {e}"),
            }
        }
    }
}
