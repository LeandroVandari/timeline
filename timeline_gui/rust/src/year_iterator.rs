use std::sync::LazyLock;

use godot::prelude::*;
use temporal_rs::Calendar;

use crate::year::{ToYear, Year};

#[derive(Debug, GodotClass)]
#[class(no_init)]
pub struct YearIterator {
    curr: temporal_rs::PlainDate,
}

impl YearIterator {
    // TODO: allow passing in desired calendar
    pub fn new(start_year: &Year) -> Option<Self> {
        Some(Self {
            curr: temporal_rs::PlainDate::try_new(start_year.get(), 1, 1, Calendar::ISO).ok()?,
        })
    }
}

#[godot_api]
impl YearIterator {
    #[func]
    fn create(start_year: Gd<Year>) -> Option<Gd<Self>> {
        Self::new(&start_year.bind()).map(Gd::from_object)
    }

    #[func]
    fn next_year(&mut self) -> Option<Gd<Year>> {
        self.next().map(Gd::from_object)
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
