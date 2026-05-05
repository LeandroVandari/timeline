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

#[godot_api]
impl Timeline {
    #[func]
    fn load_from_file(file: String) -> Option<Gd<Self>> {
        Some(Gd::from_object(Self {
            manager: serde_json::from_reader(std::io::BufReader::new(
                std::fs::File::open(std::path::Path::new(&file)).ok()?,
            ))
            .ok()?,
        }))
    }
}
