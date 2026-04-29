use godot::prelude::*;
use std::fmt::Display;

#[derive(Debug, Clone, GodotClass)]
#[class(no_init)]
pub struct Year {
    #[var(no_set)]
    year: i32,
}

impl Year {
    pub fn new(year: i32) -> Self {
        Self { year }
    }
}

#[godot_api]
impl Year {
    #[func]
    fn label(&self) -> GString {
        self.to_string().to_godot()
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.year)
    }
}
