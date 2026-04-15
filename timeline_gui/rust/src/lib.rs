use godot::prelude::*;
use timeline_core::{TimelineManager, ZonedDateTime};
mod line_marker_iterator;

struct TimelineExtension;

static YEARS_WIDTH: u32 = 20;

#[gdextension]
unsafe impl ExtensionLibrary for TimelineExtension {}

#[derive(Debug, GodotClass)]
#[class(init, base = Node)]
struct Timeline {
    manager: TimelineManager,
    leftmost_date: ZonedDateTime,
}

#[godot_api]
impl INode for Timeline {
    fn ready(&mut self) {
        self.leftmost_date = ZonedDateTime::now()
            - temporal_rs::Duration::new(YEARS_WIDTH as i64 / 2, 0, 0, 0, 0, 0, 0, 0, 0, 0)
                .unwrap();
    }
}

#[godot_api]
impl Timeline {
    #[func]
    pub fn leftmost_year(&self) -> i32 {
        self.leftmost_date.year()
    }

    #[func]
    pub fn years_width() -> u32 {
        YEARS_WIDTH
    }
}
