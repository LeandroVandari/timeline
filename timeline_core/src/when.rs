use std::{cmp::Ordering, fmt::Display};

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum When {
    Instant(ZonedDateTime),
    Period {
        start: ZonedDateTime,
        end: ZonedDateTime,
    },
}

#[derive(Debug, Clone)]
pub struct ZonedDateTime(temporal_rs::ZonedDateTime);

impl When {
    #[must_use]
    pub fn instant(when: temporal_rs::ZonedDateTime) -> Self {
        Self::Instant(when.into())
    }
}

impl PartialOrd for When {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Self::Instant(a) => match other {
                When::Period { start, end } => {
                    if matches!(a.0.compare_instant(&start.0), Ordering::Less) {
                        return Some(Ordering::Less);
                    } else if matches!(a.0.compare_instant(&end.0), Ordering::Greater) {
                        return Some(Ordering::Greater);
                    }
                    None
                }
                When::Instant(b) => Some(a.0.compare_instant(&b.0)),
            },

            Self::Period { start, end } => match other {
                Self::Instant(a) => {
                    if matches!(start.0.compare_instant(&a.0), Ordering::Greater) {
                        return Some(Ordering::Greater);
                    } else if matches!(end.0.compare_instant(&a.0), Ordering::Less) {
                        return Some(Ordering::Less);
                    }
                    None
                }
                Self::Period {
                    start: other_start,
                    end: other_end,
                } => match start.0.compare_instant(&other_start.0) {
                    Ordering::Equal => Some(end.0.compare_instant(&other_end.0)),
                    order => Some(order),
                },
            },
        }
    }
}

impl PartialEq for When {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Equal))
    }
}
impl Eq for When {}

impl Display for When {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            When::Instant(t) => write!(f, "{t}"),
            When::Period { start, end } => write!(f, "{start} - {end}"),
        }
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

#[cfg(test)]
mod tests {
    use temporal_rs::{Calendar, UtcOffset};

    use super::*;

    #[test]
    fn when_serialization() {
        for ((nanos, offset), calendar) in [0, 1774453431]
            .into_iter()
            .zip([UtcOffset::from_minutes(180), UtcOffset::from_minutes(0)].into_iter())
            .zip([Calendar::GREGORIAN, Calendar::BUDDHIST, Calendar::HEBREW].into_iter())
        {
            let before = When::instant(
                temporal_rs::ZonedDateTime::try_new(
                    nanos,
                    temporal_rs::TimeZone::UtcOffset(offset),
                    calendar,
                )
                .unwrap(),
            );
            println!("{}", serde_json::to_string(&before).unwrap());
            assert_eq!(
                before,
                serde_json::from_str(&serde_json::to_string(&before).unwrap()).unwrap()
            );
        }
    }
}
