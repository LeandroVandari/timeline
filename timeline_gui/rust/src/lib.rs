use godot::prelude::*;
use timeline_core::TimelineManager;

struct TimelineExtension;

#[gdextension]
unsafe impl ExtensionLibrary for TimelineExtension {}

#[derive(Debug, GodotClass)]
#[class(init, base = Node)]
struct Timeline {
    manager: TimelineManager,
}
