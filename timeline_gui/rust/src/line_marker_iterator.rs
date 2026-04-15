use godot::prelude::*;
use timeline_core::ZonedDateTime;

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

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, GodotConvert, Var, Export,
)]
#[godot(via=i64)]
pub enum MarkerLevel {
    #[default]
    Year = 0,
    Month = 1,
    Day = 2,
    Hour = 3,
    Minute = 4,
    Second = 5,
}

#[derive(Debug, Clone, GodotClass)]
#[class(no_init)]
pub struct LineMarker {
    level: MarkerLevel,
    #[var(no_set)]
    marker_str: GString,
}

impl Iterator for LineMarkerIterator {
    type Item = LineMarker;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr += temporal_rs::Duration::new(1, 0, 0, 0, 0, 0, 0, 0, 0, 0).ok()?;
        Some(LineMarker {
            level: MarkerLevel::Year,
            marker_str: self.curr.year().to_string().to_godot(),
        })
    }
}
