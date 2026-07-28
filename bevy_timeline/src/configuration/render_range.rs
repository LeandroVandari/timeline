use bevy::prelude::*;
use timeline_core::date_iteration::YearRange;

/// The range of years the [`RenderedTimeline`] will render.
///
/// Note that this doesn't mean all rendered years appear on screen. The actual visible year range depends
/// on the [`TimelineScreenSize`] and [`TimelineLineSeparation`]
#[derive(Debug, Component, Clone)]
#[component(on_add = super::add_rendered_timeline)]
pub struct TimelineRenderRange(pub YearRange);

impl TimelineRenderRange {
    pub fn inc(&mut self) {
        self.0.end = self.0.end.get_next().unwrap();
        self.0.start = self
            .0
            .start
            .get_next()
            .expect("since start < end, and end has a next, start should also.");
    }

    pub fn dec(&mut self) {
        self.0.start = self.0.start.get_previous().unwrap();
        self.0.end = self
            .0
            .end
            .get_previous()
            .expect("since start < end, and start has a previous, end should also.");
    }
}
