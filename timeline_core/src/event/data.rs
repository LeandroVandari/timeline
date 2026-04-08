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
    pub fn when(&self) -> &When {
        &self.when
    }

    pub fn tags(&self) -> &HashSet<TagId> {
        &self.tags
    }

    pub fn new(when: When, name: String, tags: HashSet<TagId>) -> Self {
        EventData { when, name, tags }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn add_tag(&mut self, tag: TagId) -> bool {
        self.tags.insert(tag)
    }

    pub(crate) fn remove_tag(&mut self, tag: &TagId) -> bool {
        self.tags.remove(tag)
    }
}
