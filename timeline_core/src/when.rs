use std::{cmp::Ordering, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::ZonedDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum When {
    Instant(ZonedDateTime),
    Period {
        start: ZonedDateTime,
        end: ZonedDateTime,
    },
}

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
                    if matches!(a.compare_instant(start), Ordering::Less) {
                        return Some(Ordering::Less);
                    } else if matches!(a.compare_instant(end), Ordering::Greater) {
                        return Some(Ordering::Greater);
                    }
                    None
                }
                When::Instant(b) => Some(a.compare_instant(b)),
            },

            Self::Period { start, end } => match other {
                Self::Instant(a) => {
                    if matches!(start.compare_instant(a), Ordering::Greater) {
                        return Some(Ordering::Greater);
                    } else if matches!(end.compare_instant(a), Ordering::Less) {
                        return Some(Ordering::Less);
                    }
                    None
                }
                Self::Period {
                    start: other_start,
                    end: other_end,
                } => match start.compare_instant(other_start) {
                    Ordering::Equal => Some(end.compare_instant(other_end)),
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
