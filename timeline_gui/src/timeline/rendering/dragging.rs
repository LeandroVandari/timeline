use bevy::{
    ecs::entity::{EntityHashSet, EntitySet},
    prelude::*,
    window::PrimaryWindow,
};
use tracing::instrument;

use crate::timeline::rendering::lines::{MainLine, VerticalLine, YearLabel};

#[instrument(skip_all)]
pub fn spawn_dragging_background(
    mut commands: Commands,
    render_info_query: Query<(&super::TimelineRenderInformation, &Transform)>,
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
            .observe(handle_timeline_drag)
            .id();

        commands.entity(entity).add_child(background_entity);
    }
}

type VerticalLineFilters = (With<VerticalLine>, Without<MainLine>);
type MainLineFilters = (With<MainLine>, Without<VerticalLine>);
type YearLabelsFilters = (Without<MainLine>, Without<VerticalLine>, With<YearLabel>);

/// Handle translation of the timeline's labels/lines/rendered events when it's dragged with the mouse.
#[instrument(skip_all)]
fn handle_timeline_drag(
    trigger: On<Pointer<Drag>>,

    // Traverse the relationship tree
    children_query: Query<&Children>,
    childof_query: Query<&ChildOf>,

    mut vertical_lines_query: Query<(Entity, &mut Transform), VerticalLineFilters>,
    mut main_line_query: Query<&mut Transform, MainLineFilters>,
    mut year_labels_query: Query<&mut Transform, YearLabelsFilters>,
) {
    if matches!(trigger.button, PointerButton::Primary) {
        // Since we only want to move the timeline that was dragged, the whole thing needs to traverse the relationship tree, otherwise we will edit
        // all of the timelines on screen.
        let timeline_entity = childof_query.get(trigger.entity).unwrap().parent();

        let vertical_lines = vertical_lines_query
            .iter_many_unique_mut(children_query.get(timeline_entity).unwrap().to_set());

        vertical_lines.for_each(|(entity, mut pos)| {
            pos.translation.x += trigger.delta.x;

            for mut label_pos in
                year_labels_query.iter_many_unique_mut(children_query.get(entity).unwrap().to_set())
            {
                label_pos.translation.y -= trigger.delta.y
            }
        });

        main_line_query
            .iter_many_unique_mut(children_query.get(timeline_entity).unwrap().to_set())
            .for_each(|mut pos| pos.translation.y -= trigger.delta.y);
    }
}

trait ToEntitySet {
    type Set: EntitySet;
    fn to_set(self) -> Self::Set;
}
impl ToEntitySet for &Children {
    type Set = EntityHashSet;
    // NOTE: If performance for dragging becomes a concern, I can always use an unsafe impl for `UniqueEntitySlice`, if I'm willing
    // to uphold the safety guarantees manually.
    fn to_set(self) -> Self::Set {
        EntityHashSet::from_iter(self.iter())
    }
}
