extern crate alloc;

use crate::event::{self, Event, EventData, EventId, EventKey};
use alloc::collections::BTreeSet;
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Timeline {
    /// All events sorted by their order of occurrence
    sorted_events: BTreeSet<event::EventKey>,

    events: SlotMap<event::EventId, EventData>,
}

impl Timeline {
    #[must_use]
    pub fn new() -> Self {
        Self {
            sorted_events: BTreeSet::new(),
            events: SlotMap::with_key(),
        }
    }

    pub fn insert(&mut self, data: EventData) -> EventId {
        let when = data.when().clone();
        let id = self.events.insert(data);

        assert!(
            self.sorted_events.insert(EventKey::new(id, when)),
            "inserted an event that was already in tree"
        );

        id
    }

    pub fn remove(&mut self, id: EventId) -> Option<EventData> {
        let data = self.events.remove(id)?;
        assert!(
            self.sorted_events
                .remove(&EventKey::new(id, data.when().clone())),
            "event to be removed should be present in the sorted events"
        );

        Some(data)
    }

    #[must_use]
    pub fn event_data(&self, id: EventId) -> Option<&EventData> {
        self.events.get(id)
    }

    pub fn event_data_mut(&mut self, id: EventId) -> Option<&mut EventData> {
        self.events.get_mut(id)
    }

    pub fn ordered_events(&self) -> impl Iterator<Item = Event<'_>> {
        self.sorted_events
            .iter()
            .map(|key| Event::new(key.id(), self.event_data(key.id()).unwrap()))
    }

    pub fn events(&self) -> impl Iterator<Item = Event<'_>> {
        self.events.iter().map(|(id, data)| Event::new(id, data))
    }
}
