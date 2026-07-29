//! Drag [`Children`] in [bevy](https://docs.rs/crate/bevy/latest) without changing the parent's [`Transform`].
//!
//! The goal of [`bevy_drag`](self) is allowing for fine-grained control over how [`Entities`](Entity) are moved.
//! This allows, for example, making it so a parent [`Entity`] only drags its [`Children`] when moving vertically - i.e. through [`VerticallyDraggedBy`].
//!
//! # How To Use
//! 1. **Add the [`DraggingPlugin`] to your [`App`].**
//! 2. Add [`VerticallyDraggedBy`] or [`HorizontallyDraggedBy`] or [`DraggedBy`](relationship::DraggedBy) to the entities you wish to drag.
//! 3. Emit a [`DragMessage`] when you want to drag the entities.
//! 4. Done!
//!
//! # Example
//! ## Spawning an [`Entity`] to be dragged
//! ```
//! # use bevy::prelude::*;
//! # #[derive(Component)]
//! # struct MyDraggingMarker;
//! use bevy_drag::DraggedBy;
//!
//! fn spawn_dragged(mut commands: Commands, drag_entity: Single<Entity, With<MyDraggingMarker>>) {
//!     commands.spawn((
//!         DraggedBy::new(*drag_entity)
//!     ));
//! }
//! ```
//! ## Emiting [`DragMessage`]
//! ```
//! # use bevy::prelude::*;
//! # #[derive(Component)]
//! # struct MyDraggingMarker;
//! use bevy_drag::DragMessage;
//!
//! fn emit_drag_message(mut drag_writer: MessageWriter<DragMessage>, drag_entity: Single<Entity, With<MyDraggingMarker>>) {
//!     // Could be taken from mouse movement...
//!     let delta = Vec2::new(100., 10.);
//!     drag_writer.write(DragMessage {drag_entity: *drag_entity, delta});
//! }
//!
//! ```
//!
//! ## Composing with other systems
//! Querying for groups of entities that are [`DraggedBy`] the same [`Entity`] can be done through [`HorizontallyDrags`] and [`VerticallyDrags`].
//! ```
//! # use bevy::prelude::*;
//! use bevy_drag::HorizontallyDrags;
//!
//! fn query_draggers(horizontal_drag_query: Query<(Entity, &HorizontallyDrags)>) {
//!     for (dragger, dragged) in horizontal_drag_query {
//!         println!("{dragger:?} drags {} other entities!", dragged.len());
//!     }
//! }
//! ```
//!
//! # Architecture
//! Dragging is represented through a [`Relationship`](bevy::ecs::relationship::Relationship) where entities with a `*Drags` component (`dragger`s) will
//! change the position of the corresponding `*DraggedBy` entities in the corresponding axis. This means the cost for dragging is a simple [`Query`],
//! and so it can be safely used for many entities without loss in performance compared to a manual implementation.
//!
//! In order to allow the `dragger` to not have its [`Transform`] mutated (which might cause floating point precision issues, depending on use case), dragging is triggered through [`DragMessage`].

use bevy::prelude::*;
use tracing::instrument;

pub use messages::DragMessage;
use query_ext::QueryExt as _;
pub use relationship::{
    DraggedBy, HorizontallyDraggedBy, HorizontallyDrags, VerticallyDraggedBy, VerticallyDrags,
};

mod messages;
mod relationship;

/// [`Plugin`] that enables reacting to [`DragMessage`]s.
///
/// Contains systems that handle receiving [`DragMessage`]s and update the corresponding [`*DraggedBy`](DraggedBy) [`Entitites`](Entity)' [`Transform`].
///
/// **If this [`Plugin`] is not enabled, the crate *will not work*!**
///
/// # Adding [`DraggingPlugin`] to your [`App`]
/// ```
/// # use bevy::prelude::*;
/// fn main() {
///     App::new().add_plugins(bevy_drag::DraggingPlugin);
/// }
/// ```
#[derive(Debug)]
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
