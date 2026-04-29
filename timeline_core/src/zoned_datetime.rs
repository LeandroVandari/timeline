use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub},
};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

#[derive(Debug, Clone)]
pub struct ZonedDateTime(temporal_rs::ZonedDateTime);

impl ZonedDateTime {
    pub fn now() -> Self {
        Self(
            temporal_rs::Temporal::utc_now()
                .zoned_date_time_iso(None)
                .unwrap(),
        )
    }

    pub fn year(&self) -> i32 {
        self.0.year()
    }

    pub fn month(&self) -> u8 {
        self.0.month()
    }
    pub fn day(&self) -> u8 {
        self.0.day()
    }

    pub fn hour(&self) -> u8 {
        self.0.hour()
    }

    pub fn minute(&self) -> u8 {
        self.0.minute()
    }

    pub fn second(&self) -> u8 {
        self.0.second()
    }

    pub fn round_in_place(&mut self, rounding_options: temporal_rs::options::RoundingOptions) {
        self.0 = self.0.round(rounding_options).unwrap()
    }

    pub fn compare_instant(&self, other: &Self) -> std::cmp::Ordering {
        self.0.compare_instant(&other.0)
    }
}

impl Sub<temporal_rs::Duration> for ZonedDateTime {
    type Output = Self;
    fn sub(self, rhs: temporal_rs::Duration) -> Self::Output {
        Self(self.0.subtract(&rhs, None).unwrap())
    }
}

impl Add<temporal_rs::Duration> for ZonedDateTime {
    type Output = Self;
    fn add(self, rhs: temporal_rs::Duration) -> Self::Output {
        Self(self.0.add(&rhs, None).unwrap())
    }
}

impl AddAssign<temporal_rs::Duration> for ZonedDateTime {
    fn add_assign(&mut self, rhs: temporal_rs::Duration) {
        self.0 = self.0.add(&rhs, None).unwrap()
    }
}

impl Default for ZonedDateTime {
    fn default() -> Self {
        Self(
            temporal_rs::ZonedDateTime::try_new(
                0,
                temporal_rs::TimeZone::UtcOffset(temporal_rs::UtcOffset::from_minutes(0)),
                temporal_rs::Calendar::ISO,
            )
            .unwrap(),
        )
    }
}

impl Serialize for ZonedDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use temporal_rs::options as opt;
        serializer.serialize_str(
            &self
                .0
                .to_ixdtf_string(
                    opt::DisplayOffset::Auto,
                    opt::DisplayTimeZone::Auto,
                    opt::DisplayCalendar::Always,
                    opt::ToStringRoundingOptions {
                        smallest_unit: Some(opt::Unit::Nanosecond),
                        ..Default::default()
                    },
                )
                .unwrap(),
        )
    }
}

impl<'de> Deserialize<'de> for ZonedDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use temporal_rs::options as opt;
        let s = <&str>::deserialize(deserializer)?;

        Ok(ZonedDateTime(
            temporal_rs::ZonedDateTime::from_utf8(
                s.as_bytes(),
                opt::Disambiguation::Reject,
                opt::OffsetDisambiguation::Reject,
            )
            .map_err(|e| {
                de::Error::custom(format!(
                    "invalid datetime: {e} - expected an idtfx formatted datetime"
                ))
            })?,
        ))
    }
}

impl Display for ZonedDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<temporal_rs::ZonedDateTime> for ZonedDateTime {
    fn from(value: temporal_rs::ZonedDateTime) -> Self {
        Self(value)
    }
}
