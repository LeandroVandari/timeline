use godot::prelude::*;
use std::sync::LazyLock;

use crate::year::Year;

#[derive(Debug, GodotClass)]
#[class(no_init)]
pub struct MonthIterator {
    year: Year,
    curr: temporal_rs::PlainDate,
}

impl MonthIterator {
    pub fn len(&self) -> u16 {
        self.curr.months_in_year()
    }
}

#[godot_api]
impl MonthIterator {
    #[func]
    fn _iter_init(&mut self, state: Array<Variant>) -> bool {
        self._iter_next(state)
    }

    #[func]
    fn _iter_next(&mut self, mut state: Array<Variant>) -> bool {
        if let Some(n) = self.next() {
            state.set(0, n);
            true
        } else {
            false
        }
    }

    #[func]
    fn _iter_get(&self, state: Variant) -> u8 {
        state.to()
    }
}

impl From<Year> for MonthIterator {
    fn from(year: Year) -> Self {
        Self {
            curr: temporal_rs::PlainDate::try_new(year.get(), 1, 1, temporal_rs::Calendar::ISO)
                .unwrap(),
            year,
        }
    }
}

static ONE_MONTH: LazyLock<temporal_rs::Duration> = LazyLock::new(|| {
    temporal_rs::duration::DateDuration::new(0, 1, 0, 0)
        .unwrap()
        .into()
});

impl Iterator for MonthIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.year() != self.year.get() {
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
