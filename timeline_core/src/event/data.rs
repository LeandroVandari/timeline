use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{tag::TagId, when::When};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    when: When,
    tags: HashSet<TagId>,
}

impl EventData {
    pub fn when(&self) -> &When {
        &self.when
    }

    pub fn tags(&self) -> &HashSet<TagId> {
        &self.tags
    }

    pub fn new(when: When, tags: HashSet<TagId>) -> Self {
        EventData { when, tags }
    }

    pub fn add_tag(&mut self, tag: TagId) -> bool {
        self.tags.insert(tag)
    }

    pub fn remove_tag(&mut self, tag: &TagId) -> bool {
        self.tags.remove(tag)
    }
}
