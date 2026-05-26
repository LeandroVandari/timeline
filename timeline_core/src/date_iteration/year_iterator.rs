use std::sync::LazyLock;
use temporal_rs::{Calendar, TemporalResult};

use crate::date_iteration::YearRange;

use super::year::{ToYear, Year};

#[derive(Debug)]
pub struct YearIterator {
    curr: temporal_rs::PlainDate,
    end: Option<Year>,
}

impl YearIterator {
    // TODO: allow passing in desired calendar
    pub fn new(start: &Year) -> TemporalResult<Self> {
        Ok(Self {
            curr: temporal_rs::PlainDate::try_new(start.inner(), 1, 1, Calendar::ISO)?,
            end: None,
        })
    }
}

static ONE_YEAR: LazyLock<temporal_rs::Duration> = LazyLock::new(|| {
    temporal_rs::duration::DateDuration::new(1, 0, 0, 0)
        .unwrap()
        .into()
});

impl Iterator for YearIterator {
    type Item = Year;
    fn next(&mut self) -> Option<Self::Item> {
        let year = self.curr.to_year();
        if self.end.as_ref().map(|e| &year > e).unwrap_or(false) {
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
            curr: temporal_rs::PlainDate::new(self.start.inner(), 1, 1, Calendar::ISO).unwrap(),
            end: Some(self.end.clone()),
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
