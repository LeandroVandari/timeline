use std::collections::HashSet;

use slotmap::SlotMap;
use thiserror::Error;

use crate::{
    event::{EventData, EventId},
    tag::{TagData, TagId},
    timeline::Timeline,
};

mod date_iterator;
pub mod event;
mod tag;
mod timeline;
pub mod when;

#[derive(Debug, Default)]
pub struct TimelineManager {
    timeline: Timeline,

    tags: SlotMap<TagId, TagData>,
}

impl TimelineManager {
    pub fn new() -> Self {
        Self {
            timeline: Timeline::new(),
            tags: SlotMap::with_key(),
        }
    }

    pub fn add_event(&mut self, data: EventData) -> Result<EventId, NonExistantTagsError> {
        let event_id = self.timeline.insert(data);

        let mut bad_tags = Vec::new();
        for &tag in self
            .timeline
            .event_data(event_id)
            .expect("Just added event to the timeline. It should still exist.")
            .tags()
            .iter()
        {
            if let Some(tag_data) = self.tags.get_mut(tag) {
                tag_data.add_associated_event(event_id);
            } else {
                bad_tags.push(tag);
            }
        }
        if !bad_tags.is_empty() {
            return Err(NonExistantTagsError(bad_tags));
        }
        Ok(event_id)
    }

    pub fn add_tag(&mut self, data: TagData) -> Result<TagId, NonExistantEventsError> {
        let mut bad_events = Vec::new();
        let tag_id = self.tags.insert_with_key(|tag_id| {
            for &event_id in data.associated_events() {
                if let Some(event) = self.timeline.event_data_mut(event_id) {
                    event.add_tag(tag_id);
                } else {
                    bad_events.push(event_id);
                }
            }

            data
        });

        if !bad_events.is_empty() {
            return Err(NonExistantEventsError(bad_events));
        }

        Ok(tag_id)
    }

    pub fn remove_event(&mut self, id: EventId) -> Option<EventData> {
        let data = self.timeline.remove(id)?;

        for &tag in data.tags() {
            assert!(
                self.tags
                    .get_mut(tag)
                    .expect("Tags that exist in event should be present in the tags container.")
                    .remove_associated_event(&id),
                "Tag should reflect the contained event"
            );
        }

        Some(data)
    }

    pub fn remove_tag(&mut self, id: TagId) -> Option<TagData> {
        let data = self.tags.remove(id)?;

        for &event in data.associated_events() {
            assert!(
                self.timeline
                    .event_data_mut(event)
                    .expect("Events that are referenced in the tag should exist")
                    .remove_tag(&id),
                "Event should register the contained tag"
            )
        }

        Some(data)
    }

    pub fn event_data(&self, id: EventId) -> Option<&EventData> {
        self.timeline.event_data(id)
    }

    pub fn event_data_mut(&mut self, id: EventId) -> Option<&mut EventData> {
        self.timeline.event_data_mut(id)
    }

    pub fn with_tag(&self, id: TagId) -> Option<&HashSet<EventId>> {
        self.tags.get(id).map(|data| data.associated_events())
    }
}

#[derive(Debug, Error)]
#[error("The following tags no longer exist: {0:?}")]
pub struct NonExistantTagsError(Vec<TagId>);

#[derive(Debug, Error)]
#[error("The following events no longer exist: {0:?}")]
pub struct NonExistantEventsError(Vec<EventId>);
