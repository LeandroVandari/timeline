use bevy::{prelude::*, window::WindowResized};

use crate::{configuration::TimelineScreenSize, lines::MainTimelineLine};

use super::VerticalLineRenderInfo;

pub fn update_lines_on_window_resize(
    mut resize_reader: PopulatedMessageReader<WindowResized>,
    mut meshes: ResMut<Assets<Mesh>>,

    mut timeline_info_query: Query<&mut VerticalLineRenderInfo, Without<TimelineScreenSize>>,

    mut main_line_query: Query<(&ChildOf, &mut Mesh2d), With<MainTimelineLine>>,
) {
    for resize in resize_reader.read() {
        // Horizontal lines
        for (&ChildOf(timeline_entity), line) in main_line_query.iter_mut() {
            if timeline_info_query.contains(timeline_entity) {
                *meshes.get_mut(line.0.id()).unwrap() = Rectangle::new(resize.width, 3.).into();
            }
        }

        // Vertical Lines
        for vertical_line_render_info in timeline_info_query.iter_mut() {
            *meshes.get_mut(vertical_line_render_info.mesh.id()).unwrap() =
                Rectangle::new(1., resize.height).into();
        }
    }
}
