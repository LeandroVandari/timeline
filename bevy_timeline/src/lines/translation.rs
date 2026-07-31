use bevy::prelude::*;
use bevy_drag::DragMessage;

use bevy_zoom::{ZoomMessage, ZoomSet};

use crate::configuration::{TimelineHorizontalOffset, TimelineVerticalOffset};

pub struct LineTranslationPlugin;

impl Plugin for LineTranslationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                Self::update_offset_on_drag,
                Self::update_offset_on_zoom.after(ZoomSet),
            )
                .in_set(LineTranslationSet),
        );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, SystemSet)]
pub struct LineTranslationSet;

impl LineTranslationPlugin {
    fn update_offset_on_drag(
        mut drag_messages: PopulatedMessageReader<DragMessage>,
        mut timeline_offset_query: Query<(
            &mut TimelineHorizontalOffset,
            &mut TimelineVerticalOffset,
        )>,
    ) {
        for msg in drag_messages.read() {
            let Ok((mut h_offset, mut v_offset)) = timeline_offset_query.get_mut(msg.drag_entity)
            else {
                continue;
            };
            **h_offset += msg.delta.x;
            **v_offset += msg.delta.y;
        }
    }

    fn update_offset_on_zoom(
        mut zoom_messages: PopulatedMessageReader<ZoomMessage>,
        mut offset_query: Query<(&mut TimelineHorizontalOffset, &mut TimelineVerticalOffset)>,
    ) {
        for message in zoom_messages.read() {
            match offset_query.get_mut(message.entity()) {
                Ok((mut h_offset, mut v_offset)) => {
                    **h_offset = message.anchor().x.lerp(**h_offset, message.factor());
                    **v_offset = message.anchor().y.lerp(**v_offset, message.factor());
                }
                Err(e) => {
                    error!("Couldn't update timeline offset on zoom: {e}");
                }
            }
        }
    }
}
