use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{tag::TagId, when::When};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    when: When,
    name: String,
    tags: HashSet<TagId>,
}

impl EventData {
    #[must_use]
    pub fn when(&self) -> &When {
        &self.when
    }

    #[must_use]
    pub fn tags(&self) -> &HashSet<TagId> {
        &self.tags
    }

    #[must_use]
    pub fn new(when: When, name: String, tags: HashSet<TagId>) -> Self {
        EventData { when, name, tags }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn add_tag(&mut self, tag: TagId) -> bool {
        self.tags.insert(tag)
    }

    pub(crate) fn remove_tag(&mut self, tag: TagId) -> bool {
        self.tags.remove(&tag)
    }
}
