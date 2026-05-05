use std::fmt::Display;

use temporal_rs::{TemporalError, TemporalResult};

use super::{month_iterator::MonthIterator, year_iterator::YearIterator};

#[derive(Debug, Clone)]
pub struct Year(i32);

impl Year {
    pub fn current() -> TemporalResult<Self> {
        Ok(Self(
            temporal_rs::Temporal::utc_now()
                .plain_date_iso(None)?
                .year(),
        ))
    }

    pub fn inner(&self) -> i32 {
        self.0
    }

    fn get_next(&self) -> TemporalResult<Self> {
        let mut temp_iter = YearIterator::new(self)?;
        temp_iter.nth(1).ok_or(TemporalError::abrupt_end())
    }

    fn get_previous(&self) -> TemporalResult<Self> {
        let mut temp_iter = YearIterator::new(self)?;
        temp_iter.nth_back(1).ok_or(TemporalError::abrupt_end())
    }

    fn months(&self) -> MonthIterator {
        self.clone().into()
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait ToYear {
    fn to_year(&self) -> Year;
}

impl ToYear for temporal_rs::PlainDate {
    fn to_year(&self) -> Year {
        Year(self.year())
    }
}
