use std::collections::HashSet;

use slotmap::new_key_type;

use crate::event::EventId;

new_key_type! {
    pub struct TagId;
}

#[derive(Debug)]
pub struct TagData {
    associated_events: HashSet<EventId>,
    name: String,
}
impl TagData {
    pub fn new(name: String) -> Self {
        Self {
            associated_events: HashSet::new(),
            name,
        }
    }

    pub fn add_associated_event(&mut self, id: EventId) {
        self.associated_events.insert(id);
    }

    pub fn remove_associated_event(&mut self, id: &EventId) -> bool {
        self.associated_events.remove(id)
    }

    pub fn associated_events(&self) -> &HashSet<EventId> {
        &self.associated_events
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
