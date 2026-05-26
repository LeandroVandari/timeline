use temporal_rs::{Calendar, options::DifferenceSettings};

use crate::date_iteration::year::Year;

#[derive(Debug, Clone)]
pub struct YearRange {
    pub start: Year,
    pub end: Year,
}

impl YearRange {
    pub fn len(&self) -> usize {
        let mut settings = DifferenceSettings::default();
        settings.smallest_unit = Some(temporal_rs::options::Unit::Year);
        temporal_rs::PlainDate::new(self.end.inner(), 1, 1, Calendar::ISO)
            .unwrap()
            .since(
                &temporal_rs::PlainDate::new(self.start.inner(), 1, 1, Calendar::ISO).unwrap(),
                settings,
            )
            .unwrap()
            .years() as usize
    }
}
