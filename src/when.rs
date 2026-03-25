use std::cmp::Ordering;

use serde::{
    Deserialize, Serialize, Serializer,
    de::{self, Visitor},
    ser::SerializeStruct,
};
use temporal_rs::ZonedDateTime;

#[derive(Debug, Clone)]
pub struct When {
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

impl Serialize for When {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("When", 1)?;
        use temporal_rs::options as opt;
        state.serialize_field(
            "when",
            &self
                .when
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
        )?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for When {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct WhenVisitor;

        impl<'de> Visitor<'de> for WhenVisitor {
            type Value = When;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("When struct with a field that contains a Temporal Timestamp")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                use temporal_rs::options as opt;
                if let Some(("when", when)) = map.next_entry::<&str, &str>()? {
                    Ok(When {
                        when: ZonedDateTime::from_utf8(
                            when.as_bytes(),
                            opt::Disambiguation::Reject,
                            opt::OffsetDisambiguation::Reject,
                        )
                        .map_err(|e| {
                            de::Error::custom(format!(
                                "invalid datetime: {e} - expected an idtfx formatted datetime"
                            ))
                        })?,
                    })
                } else {
                    Err(de::Error::missing_field("when"))
                }
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                use temporal_rs::options as opt;
                let when_str: String = seq
                    .next_element()?
                    .ok_or(de::Error::missing_field("when"))?;

                Ok(When {
                    when: ZonedDateTime::from_utf8(
                        when_str.as_bytes(),
                        opt::Disambiguation::Reject,
                        opt::OffsetDisambiguation::Reject,
                    )
                    .map_err(|e| {
                        de::Error::custom(format!(
                            "invalid datetime: {e} - expected an idtfx formatted datetime"
                        ))
                    })?,
                })
            }
        }

        deserializer.deserialize_struct("When", &["when"], WhenVisitor)
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
