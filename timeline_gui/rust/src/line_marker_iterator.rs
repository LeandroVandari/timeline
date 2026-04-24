use godot::prelude::*;
use timeline_core::ZonedDateTime;

use crate::{line_marker::LineMarker, marker_level::MarkerLevel};

#[derive(Debug, GodotClass)]
#[class(no_init)]
pub struct LineMarkerIterator {
    curr: ZonedDateTime,
    curr_level: MarkerLevel,
    max_level: MarkerLevel,
}

#[godot_api]
impl LineMarkerIterator {
    #[func]
    fn create(start_date: String, max_level: MarkerLevel) -> Option<Gd<Self>> {
        let date = match temporal_rs::ZonedDateTime::from_utf8(
            start_date.as_bytes(),
            temporal_rs::options::Disambiguation::Reject,
            temporal_rs::options::OffsetDisambiguation::Reject,
        ) {
            Ok(date) => date,
            Err(e) => {
                godot_error!("Couldn't create datetime: {e}");
                return None;
            }
        };
        Some(Gd::from_object(Self {
            curr: date.into(),

            curr_level: MarkerLevel::Year,
            max_level,
        }))
    }

    #[func]
    fn create_from_now(max_level: MarkerLevel) -> Gd<Self> {
        Gd::from_object(Self {
            curr: ZonedDateTime::now(),
            max_level,
            curr_level: MarkerLevel::Year,
        })
    }

    #[func]
    fn next_marker(&mut self) -> Option<Gd<LineMarker>> {
        self.next().map(Gd::from_object)
    }
}

impl Iterator for LineMarkerIterator {
    type Item = LineMarker;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr += self.curr_level.as_duration();
        let marker = LineMarker::new(self.curr_level, &self.curr);

        if self.curr_level != self.max_level {
            self.curr_level = (self.curr_level as u8 + 1)
                .try_into()
                .expect("Since it's not the max level, we can go further");
        } else {
            self.curr_level = MarkerLevel::Year;
        }

        Some(marker)
    }
}
