use std::cmp::Ordering;

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self},
};
use temporal_rs::ZonedDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct When {
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    when: temporal_rs::ZonedDateTime,
}

impl When {
    pub fn new(when: temporal_rs::ZonedDateTime) -> Self {
        Self { when }
    }
    pub fn compare_instant(&self, other: &Self) -> Ordering {
        self.when.compare_instant(&other.when)
    }
}

impl PartialEq for When {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.compare_instant(other), Ordering::Equal)
    }
}

impl Eq for When {}

fn serialize_dt<S>(dt: &ZonedDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use temporal_rs::options as opt;
    serializer.serialize_str(
        &dt.to_ixdtf_string(
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

fn deserialize_dt<'de, D>(deserializer: D) -> Result<ZonedDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    use temporal_rs::options as opt;
    let s = <&str>::deserialize(deserializer)?;

    temporal_rs::ZonedDateTime::from_utf8(
        s.as_bytes(),
        opt::Disambiguation::Reject,
        opt::OffsetDisambiguation::Reject,
    )
    .map_err(|e| {
        de::Error::custom(format!(
            "invalid datetime: {e} - expected an idtfx formatted datetime"
        ))
    })
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
            let before = When::new(
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
