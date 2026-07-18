use std::sync::LazyLock;

use super::year::Year;

#[derive(Debug)]
pub struct MonthIterator {
    year: Year,
    curr: temporal_rs::PlainDate,
}

impl From<Year> for MonthIterator {
    fn from(year: Year) -> Self {
        Self {
            curr: year.as_date(),
            year,
        }
    }
}

static ONE_MONTH: LazyLock<temporal_rs::Duration> = LazyLock::new(|| {
    temporal_rs::duration::DateDuration::new(0, 1, 0, 0)
        .expect("Always valid.")
        .into()
});

impl Iterator for MonthIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.year() != self.year.inner() {
            return None;
        }

        let month = self.curr.month();

        self.curr = self
            .curr
            .add(&ONE_MONTH, Some(temporal_rs::options::Overflow::Reject))
            .ok()?;

        Some(month)
    }
}
