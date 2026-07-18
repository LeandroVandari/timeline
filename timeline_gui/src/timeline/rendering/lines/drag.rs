use bevy::prelude::*;
use tracing::instrument;

use crate::{dragging::DragMessage, timeline::rendering::configuration::TimelineHorizontalOffset};

#[instrument(skip_all)]
pub fn update_timeline_offset_on_drag(
    mut drag_messages: MessageReader<DragMessage>,
    mut timeline_offset_query: Query<&mut TimelineHorizontalOffset>,
) {
    for msg in drag_messages.read() {
        **timeline_offset_query.get_mut(msg.entity()).unwrap() += msg.delta().x;
    }
}
