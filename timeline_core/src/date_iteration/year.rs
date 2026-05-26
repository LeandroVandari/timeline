use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use temporal_rs::{TemporalError, TemporalResult, partial::PartialDuration};

use super::{month_iterator::MonthIterator, year_iterator::YearIterator};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
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

    pub fn get_next(&self) -> TemporalResult<Self> {
        let mut temp_iter = YearIterator::new(self)?;
        temp_iter.nth(1).ok_or(TemporalError::abrupt_end())
    }

    pub fn get_previous(&self) -> TemporalResult<Self> {
        let mut temp_iter = YearIterator::new(self)?;
        temp_iter.nth_back(1).ok_or(TemporalError::abrupt_end())
    }

    fn months(&self) -> MonthIterator {
        self.clone().into()
    }
}

impl Sub<i64> for Year {
    type Output = Self;
    fn sub(self, rhs: i64) -> Self::Output {
        temporal_rs::PlainDate::new_iso(self.0, 1, 1)
            .unwrap()
            .subtract(
                &temporal_rs::Duration::from_partial_duration(
                    PartialDuration::empty().with_years(rhs),
                )
                .unwrap(),
                Some(temporal_rs::options::Overflow::Reject),
            )
            .unwrap()
            .to_year()
    }
}

impl Add<i64> for Year {
    type Output = Self;
    fn add(self, rhs: i64) -> Self::Output {
        temporal_rs::PlainDate::new_iso(self.0, 1, 1)
            .unwrap()
            .add(
                &temporal_rs::Duration::from_partial_duration(
                    PartialDuration::empty().with_years(rhs),
                )
                .unwrap(),
                Some(temporal_rs::options::Overflow::Reject),
            )
            .unwrap()
            .to_year()
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
