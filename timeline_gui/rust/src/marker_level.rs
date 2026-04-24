use godot::prelude::*;
use thiserror::Error;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, GodotConvert, Var, Export,
)]
#[repr(u8)]
#[godot(via=u8)]
pub enum MarkerLevel {
    #[default]
    Year = 0,
    Month = 1,
    Day = 2,
    Hour = 3,
    Minute = 4,
    Second = 5,
}

impl MarkerLevel {
    pub fn as_duration(&self) -> temporal_rs::Duration {
        match self {
            Self::Year => temporal_rs::Duration::new(1, 0, 0, 0, 0, 0, 0, 0, 0, 0),
            Self::Month => temporal_rs::Duration::new(0, 1, 0, 0, 0, 0, 0, 0, 0, 0),
            Self::Day => temporal_rs::Duration::new(0, 0, 0, 1, 0, 0, 0, 0, 0, 0),
            Self::Hour => temporal_rs::Duration::new(0, 0, 0, 0, 1, 0, 0, 0, 0, 0),
            Self::Minute => temporal_rs::Duration::new(0, 0, 0, 0, 0, 1, 0, 0, 0, 0),
            Self::Second => temporal_rs::Duration::new(0, 0, 0, 0, 0, 0, 1, 0, 0, 0),
        }
        .unwrap()
    }
}

#[derive(Debug, Error)]
#[error("Invalid MarkerLevel: {0}")]
pub struct InvalidMarkerLevelError(u8);
impl TryFrom<u8> for MarkerLevel {
    type Error = InvalidMarkerLevelError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Year,
            1 => Self::Month,
            2 => Self::Day,
            3 => Self::Hour,
            4 => Self::Minute,
            5 => Self::Second,
            _ => return Err(InvalidMarkerLevelError(value)),
        })
    }
}
