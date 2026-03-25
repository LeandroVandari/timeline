mod data;
pub use data::EventData;
mod id;
use std::collections::HashSet;

pub use id::EventId;

mod key;
pub use key::EventKey;

use crate::{tag::TagId, when::When};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    id: EventId,
    data: EventData,
}

impl Event {
    pub fn key(&self) -> EventKey {
        self.into()
    }

    pub fn id(&self) -> EventId {
        self.id
    }

    pub fn when(&self) -> &When {
        self.data.when()
    }

    pub fn tags(&self) -> &HashSet<TagId> {
        self.data.tags()
    }
}
