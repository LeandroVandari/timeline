use godot::{classes::InputEvent, prelude::*};
use timeline_core::TimelineManager;

struct TimelineExtension;

#[gdextension]
unsafe impl ExtensionLibrary for TimelineExtension {}

#[derive(Debug, GodotClass)]
#[class(init, base = Node)]
struct Timeline {
    manager: TimelineManager,
    visible_years: Vec<i16>,
}

#[godot_api]
impl INode for Timeline {
    fn ready(&mut self) {
        self.visible_years = (-5..5).collect();
    }

    fn process(&mut self, delta: f32) {
        godot_print!("{:?}", self.visible_years)
    }

    fn input(&mut self, input: Gd<InputEvent>) {
        if input.is_action("timeline_drag") {
            self.visible_years.iter_mut().for_each(|y| *y += 1);
        }
    }
}
