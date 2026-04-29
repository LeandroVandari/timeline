use godot::prelude::*;
use temporal_rs::Calendar;

use crate::year::Year;

#[derive(Debug, GodotClass)]
#[class(no_init)]
pub struct YearIterator {
    curr: temporal_rs::PlainDate,
}

#[godot_api]
impl YearIterator {
    #[func]
    fn create(start_year: i32) -> Option<Gd<Self>> {
        Some(Gd::from_object(Self {
            // TODO: allow passing in desired calendar through godot
            curr: temporal_rs::PlainDate::new(start_year, 0, 0, Calendar::ISO).ok()?,
        }))
    }

    #[func]
    fn create_from_now() -> Option<Gd<Self>> {
        Self::create(
            temporal_rs::Temporal::utc_now()
                .plain_date_iso(None)
                .ok()?
                .year(),
        )
    }

    #[func]
    fn next_year(&mut self) -> Option<Gd<Year>> {
        self.next().map(Gd::from_object)
    }
}

impl Iterator for YearIterator {
    type Item = Year;
    fn next(&mut self) -> Option<Self::Item> {
        let year = Year::new(self.curr.year());
        self.curr = self
            .curr
            .add(
                &temporal_rs::duration::DateDuration::new(1, 0, 0, 0)
                    .ok()?
                    .into(),
                Some(temporal_rs::options::Overflow::Reject),
            )
            .ok()?;

        Some(year)
    }
}
