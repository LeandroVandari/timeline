use bevy::prelude::*;
use tracing::instrument;

use crate::configuration::{TimelineHorizontalOffset, TimelineVerticalOffset};
use bevy_drag::DragMessage;

#[instrument(skip_all)]
pub fn update_timeline_offset_on_drag(
    mut drag_messages: PopulatedMessageReader<DragMessage>,
    mut timeline_offset_query: Query<(&mut TimelineHorizontalOffset, &mut TimelineVerticalOffset)>,
) {
    for msg in drag_messages.read() {
        let Ok((mut h_offset, mut v_offset)) = timeline_offset_query.get_mut(msg.entity()) else {
            continue;
        };
        **h_offset += msg.delta().x;
        **v_offset -= msg.delta().y;
    }
}
