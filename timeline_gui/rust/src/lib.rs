use godot::prelude::*;
use timeline_core::TimelineManager;

mod year;
mod year_iterator;
struct TimelineExtension;

static YEARS_WIDTH: u32 = 20;

#[gdextension]
unsafe impl ExtensionLibrary for TimelineExtension {}

#[derive(Debug, GodotClass)]
#[class(init, base = Node)]
struct Timeline {
    manager: TimelineManager,
}

#[godot_api]
impl Timeline {
    #[func]
    pub fn years_width() -> u32 {
        YEARS_WIDTH
    }
}
