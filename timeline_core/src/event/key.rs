use serde::{Deserialize, Serialize};

use crate::when::When;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventKey {
    id: super::EventId,
    when: When,
}

impl EventKey {
    pub(crate) fn new(id: super::EventId, when: When) -> Self {
        Self { id, when }
    }

    pub fn id(&self) -> super::EventId {
        self.id
    }
}

impl PartialOrd for EventKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EventKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.when.partial_cmp(&other.when) {
            None | Some(std::cmp::Ordering::Equal) => self.id.cmp(&other.id),
            Some(order) => order,
        }
    }
}
