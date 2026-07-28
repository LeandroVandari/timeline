use bevy::prelude::*;

#[derive(Debug, Component, Clone, Copy, Deref, DerefMut)]
#[component(on_add = super::add_rendered_timeline)]
pub struct TimelineHorizontalOffset(f32);

#[derive(Debug, Component, Clone, Copy, Deref, DerefMut)]
#[component(on_add = super::add_rendered_timeline)]
pub struct TimelineVerticalOffset(f32);

impl TimelineHorizontalOffset {
    pub const ZERO: Self = Self(0.);
}
impl TimelineVerticalOffset {
    pub const ZERO: Self = Self(0.);
}
