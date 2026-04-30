use godot::prelude::*;
use timeline_core::TimelineManager;

mod year;
mod year_iterator;

mod month_iterator;

struct TimelineExtension;

#[gdextension]
unsafe impl ExtensionLibrary for TimelineExtension {}

#[derive(Debug, GodotClass)]
#[class(init, base = Node)]
struct Timeline {
    manager: TimelineManager,
}
