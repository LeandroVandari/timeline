#![expect(clippy::needless_pass_by_value, reason = "Bevy Queries")]

#[cfg(feature = "debug")]
pub mod debug;
pub mod setup;
pub mod timeline;
