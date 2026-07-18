use bevy::prelude::*;

mod event;
mod group;
mod info;
mod system_set;

pub use event::WrapAroundEvent;
pub use info::WrapAroundInfo;
pub use system_set::WrapAroundSet;

use group::WrapAroundGroup;

pub struct WrapAroundPlugin;

impl Plugin for WrapAroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            Self::handle_wrap_around.in_set(system_set::WrapAroundSet),
        );
    }
}

impl WrapAroundPlugin {
    fn handle_wrap_around(
        moved_query: Query<(&mut Transform, &WrapAround, Entity), Changed<Transform>>,
        wrap_info_query: Query<&WrapAroundInfo>,
        mut commands: Commands,
    ) {
        for (mut pos, WrapAround(wrap_entity), entity) in moved_query {
            let Ok(&WrapAroundInfo {
                center,
                half_width,
                emit_message,
            }) = wrap_info_query.get(*wrap_entity)
            else {
                error!(
                    "Couldn't get information to wrap around. That is probably because target `WrapAroundGroup` has had its `WrapAroundInfo` component removed."
                );
                continue;
            };

            let diff_center = pos.translation.x - center;
            if diff_center.abs() > half_width {
                // Wrap it around by adding or subtracting a width
                pos.translation.x =
                    (half_width * 2.).mul_add(-diff_center.signum(), pos.translation.x);

                if emit_message {
                    commands.trigger(WrapAroundEvent {
                        entity,
                        direction: if diff_center.is_sign_positive() {
                            WrapDirection::Right
                        } else {
                            WrapDirection::Left
                        },
                    });
                }
            }
        }
    }
}

#[derive(Debug, Component)]
#[relationship(relationship_target = WrapAroundGroup)]
pub struct WrapAround(pub Entity);

#[derive(Debug)]
pub enum WrapDirection {
    Left,
    Right,
}
