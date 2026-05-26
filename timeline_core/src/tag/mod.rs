use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use slotmap::new_key_type;

use crate::event::EventId;

new_key_type! {
    pub struct TagId;
}

#[expect(
    clippy::module_name_repetitions,
    reason = "TagData should be `use`d and would be too generic if it didn't specify what the data refers to."
)]
#[derive(Debug, Serialize, Deserialize)]
pub struct TagData {
    associated_events: HashSet<EventId>,
    name: String,
}
impl TagData {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            associated_events: HashSet::new(),
            name,
        }
    }

    pub fn add_associated_event(&mut self, id: EventId) {
        self.associated_events.insert(id);
    }

    pub fn remove_associated_event(&mut self, id: EventId) -> bool {
        self.associated_events.remove(&id)
    }

    #[must_use]
    pub fn associated_events(&self) -> &HashSet<EventId> {
        &self.associated_events
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}
