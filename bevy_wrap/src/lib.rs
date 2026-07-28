use bevy::prelude::*;

mod group;
mod info;
mod message;
mod system_set;

pub use info::WrapAroundInfo;
pub use message::WrapAroundMessage;
pub use system_set::WrapAroundSet;

use group::WrapAroundGroup;

pub struct WrapAroundPlugin;

impl Plugin for WrapAroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            Self::handle_wrap_around.in_set(system_set::WrapAroundSet),
        )
        .add_message::<WrapAroundMessage>();
    }
}

impl WrapAroundPlugin {
    fn handle_wrap_around(
        moved_query: Query<(&mut Transform, &WrapAround, Entity), Changed<Transform>>,
        wrap_info_query: Query<&WrapAroundInfo>,
        mut wrap_around_message_writer: MessageWriter<WrapAroundMessage>,
    ) {
        wrap_around_message_writer.write_batch(moved_query.into_iter().filter_map(|(mut pos, WrapAround(wrap_entity), entity)| {
            let Ok(&WrapAroundInfo {
                center,
                half_width,
                emit_message,
            }) = wrap_info_query.get(*wrap_entity)
            else {
                error!(
                    "Couldn't get information to wrap around. That is probably because target `WrapAroundGroup` has had its `WrapAroundInfo` component removed."
                );
                return None;
            };
            
            let diff_center = pos.translation.x - center;
            if diff_center.abs() > half_width {
                // Wrap it around by adding or subtracting a width
                pos.translation.x =
                    (half_width * 2.).mul_add(-diff_center.signum(), pos.translation.x);
                if emit_message {
                    return Some(WrapAroundMessage {
                        entity,
                        direction: if diff_center.is_sign_positive() {
                            WrapDirection::Right
                        } else {
                            WrapDirection::Left
                        },
                    });
                    
                }
            }
            None
        }));
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
