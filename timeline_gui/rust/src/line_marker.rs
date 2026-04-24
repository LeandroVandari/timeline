use godot::prelude::*;
use timeline_core::ZonedDateTime;

use crate::marker_level::MarkerLevel;

#[derive(Debug, Clone, GodotClass)]
#[class(no_init)]
pub struct LineMarker {
    #[var(no_set)]
    level: MarkerLevel,
    #[var(no_set)]
    marker_str: GString,
}

impl LineMarker {
    pub fn new(level: MarkerLevel, dt: &ZonedDateTime) -> Self {
        let marker_str = match level {
            MarkerLevel::Year => dt.year().to_string(),
            MarkerLevel::Month => dt.month().to_string(),
            MarkerLevel::Day => {
                format!("{}/{}", dt.day(), dt.month())
            }
            MarkerLevel::Hour => dt.hour().to_string(),
            MarkerLevel::Minute => {
                format!("{}:{}", dt.hour(), dt.minute())
            }
            MarkerLevel::Second => {
                format!("{}:{}:{}", dt.hour(), dt.minute(), dt.second())
            }
        }
        .to_godot();

        Self { level, marker_str }
    }
}
