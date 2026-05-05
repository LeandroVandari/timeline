use std::sync::LazyLock;
use temporal_rs::{Calendar, TemporalResult};

use super::year::{ToYear, Year};

#[derive(Debug)]
pub struct YearIterator {
    curr: temporal_rs::PlainDate,
}

impl YearIterator {
    // TODO: allow passing in desired calendar
    pub fn new(start_year: &Year) -> TemporalResult<Self> {
        Ok(Self {
            curr: temporal_rs::PlainDate::try_new(start_year.inner(), 1, 1, Calendar::ISO)?,
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
        self.curr = self
            .curr
            .add(&ONE_YEAR, Some(temporal_rs::options::Overflow::Reject))
            .ok()?;

        Some(year)
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
