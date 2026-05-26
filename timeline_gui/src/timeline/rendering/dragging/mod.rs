use bevy::{ecs::entity::EntityHashSet, prelude::*};
use tracing::instrument;

use crate::timeline::rendering::dragging::relationship::{
    HorizontallyDraggedBy, HorizontallyDrags, VerticallyDraggedBy, VerticallyDrags,
};
pub use messages::{DragMessage, WrapAround, WrapDirection};

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
            (&mut Transform, Option<&HorizontalWrapAround>, Entity),
            Or<(With<VerticallyDraggedBy>, With<HorizontallyDraggedBy>)>,
        >,

        vertically_drags_query: Query<&VerticallyDrags>,
        horizontally_drags_query: Query<&HorizontallyDrags>,

        mut commands: Commands,
    ) {
        for drag_message in drag_messages.read() {
            let dragged_entity = drag_message.entity();
            let delta = drag_message.delta();

            match vertically_drags_query.get(dragged_entity) {
                Ok(dragged) => {
                    drag_query
                        .iter_many_unique_mut(EntityHashSet::from_iter(
                            dragged.collection().iter().copied(),
                        ))
                        .for_each(|(mut pos, ..)| pos.translation.y -= delta.y);
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
                        .for_each(|(mut pos, infinite_drag, entity)| {
                            pos.translation.x += delta.x;
                            if let Some(&HorizontalWrapAround {
                                center,
                                half_width,
                                emit_message,
                            }) = infinite_drag
                                && (pos.translation.x - center) * delta.x.signum() > half_width
                            {
                                // Wrap it around by adding or subtracting a width
                                pos.translation.x =
                                    (half_width * 2.).mul_add(-delta.x.signum(), pos.translation.x);

                                if emit_message {
                                    commands.trigger(WrapAround {
                                        entity,
                                        direction: if delta.x > 0. {
                                            WrapDirection::Right
                                        } else {
                                            WrapDirection::Left
                                        },
                                    });
                                }
                            }
                        });
                }
                Err(bevy::ecs::query::QueryEntityError::QueryDoesNotMatch(_, _)) => (),
                Err(e) => error!("Error running drag query: {e}"),
            }
        }
    }
}

#[derive(Debug, Component)]
// TODO: Generalize to vertical drag aswell.
pub struct HorizontalWrapAround {
    pub center: f32,
    pub half_width: f32,
    pub emit_message: bool,
}
