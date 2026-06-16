use core::{
    fmt::Display,
    ops::{Add, Sub},
};

use temporal_rs::{Calendar, PlainDate, TemporalError, TemporalResult, partial::PartialDuration};

use super::{month_iterator::MonthIterator, year_iterator::YearIterator};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Year(i32);

impl Year {
    pub fn current() -> TemporalResult<Self> {
        Ok(Self::from(
            temporal_rs::Temporal::utc_now().plain_date_iso(None)?,
        ))
    }

    #[must_use]
    pub fn inner(&self) -> i32 {
        self.0
    }

    pub fn get_next(&self) -> TemporalResult<Self> {
        let mut temp_iter = YearIterator::new(self);
        temp_iter.nth(1).ok_or(TemporalError::abrupt_end())
    }

    pub fn get_previous(&self) -> TemporalResult<Self> {
        let mut temp_iter = YearIterator::new(self);
        temp_iter.nth_back(1).ok_or(TemporalError::abrupt_end())
    }

    #[must_use]
    pub fn months(&self) -> MonthIterator {
        self.clone().into()
    }

    #[must_use]
    pub fn as_date(&self) -> PlainDate {
        PlainDate::try_new(self.0, 1, 1, Calendar::ISO).expect("Works as long as `self` is valid.")
    }
}

impl Sub<i64> for Year {
    type Output = Self;
    fn sub(self, rhs: i64) -> Self::Output {
        self.as_date()
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
        self.as_date()
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

impl From<PlainDate> for Year {
    fn from(value: PlainDate) -> Self {
        Self(value.year())
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
