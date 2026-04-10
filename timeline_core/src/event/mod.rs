mod data;
use std::ops::Deref;

pub use data::EventData;
mod id;

pub use id::EventId;

mod key;
pub use key::EventKey;

pub struct Event<'a> {
    id: EventId,
    data: &'a EventData,
}

impl<'a> Event<'a> {
    pub(crate) fn new(id: EventId, data: &'a EventData) -> Self {
        Self { id, data }
    }

    #[must_use]
    pub fn id(&self) -> EventId {
        self.id
    }
}

impl Deref for Event<'_> {
    type Target = EventData;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}
