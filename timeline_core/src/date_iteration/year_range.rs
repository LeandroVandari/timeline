use temporal_rs::options::DifferenceSettings;

use crate::date_iteration::{YearIterator, year::Year};

#[derive(Debug, Clone)]
pub struct YearRange {
    pub start: Year,
    pub end: Year,
}

impl YearRange {
    #[must_use]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "won't truncate on 32 bit targets because it's always positive"
    )]
    pub fn len(&self) -> usize {
        let mut settings = DifferenceSettings::default();
        settings.smallest_unit = Some(temporal_rs::options::Unit::Year);

        (self
            .end
            .as_date()
            .since(&self.start.as_date(), settings)
            .expect("Same calendar and settings are valid.")
            .years()
            + 1)
        .max(0)
        .cast_unsigned() as usize
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.end <= self.start
    }

    #[must_use]
    pub fn iter(&self) -> YearIterator {
        self.into_iter()
    }
}
