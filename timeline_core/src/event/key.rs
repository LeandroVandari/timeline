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

    #[must_use]
    pub fn id(&self) -> super::EventId {
        self.id
    }
}

impl PartialOrd for EventKey {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EventKey {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match self.when.partial_cmp(&other.when) {
            None | Some(core::cmp::Ordering::Equal) => self.id.cmp(&other.id),
            Some(order) => order,
        }
    }
}
