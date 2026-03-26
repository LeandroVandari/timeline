use crate::{event::Event, when::When};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventKey {
    id: super::EventId,
    when: When,
}

impl EventKey {
    pub(crate) fn new(id: super::EventId, when: When) -> Self {
        Self { id, when }
    }
}

impl From<&Event> for EventKey {
    fn from(e: &Event) -> Self {
        Self::new(e.id(), e.when().clone())
    }
}

impl PartialOrd for EventKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EventKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.when
            .compare_instant(&other.when)
            .then_with(|| self.id.cmp(&other.id))
    }
}
