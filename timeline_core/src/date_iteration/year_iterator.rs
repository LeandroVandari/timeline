use std::sync::LazyLock;

use crate::date_iteration::YearRange;

use super::year::{ToYear as _, Year};

#[derive(Debug, Clone)]
pub struct YearIterator {
    curr: temporal_rs::PlainDate,
    end: Option<Year>,
}

impl YearIterator {
    // TODO: allow passing in desired calendar
    #[must_use]
    pub fn new(start: &Year) -> Self {
        Self {
            curr: start.as_date(),
            end: None,
        }
    }
}

static ONE_YEAR: LazyLock<temporal_rs::Duration> = LazyLock::new(|| {
    temporal_rs::duration::DateDuration::new(1, 0, 0, 0)
        .expect("Always valid.")
        .into()
});

impl Iterator for YearIterator {
    type Item = Year;
    fn next(&mut self) -> Option<Self::Item> {
        let year = self.curr.to_year();
        if self.end.as_ref().is_some_and(|end| &year > end) {
            return None;
        }

        self.curr = self
            .curr
            .add(&ONE_YEAR, Some(temporal_rs::options::Overflow::Reject))
            .ok()?;

        Some(year)
    }
}

impl IntoIterator for &YearRange {
    type IntoIter = YearIterator;
    type Item = Year;
    fn into_iter(self) -> Self::IntoIter {
        YearIterator {
            curr: self.start.as_date(),
            end: Some(self.end),
        }
    }
}

impl DoubleEndedIterator for YearIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        let year = self.curr.to_year();
        self.curr = self
            .curr
            .subtract(&ONE_YEAR, Some(temporal_rs::options::Overflow::Reject))
            .ok()?;

        Some(year)
    }
}
