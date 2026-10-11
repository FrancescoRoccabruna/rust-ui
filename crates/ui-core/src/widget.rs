use ui_render::{DrawCommand, Rect, Size};

use crate::Constraints;

pub trait Widget {
    fn measure(&self, constraints: Constraints) -> Size;

    fn draw(
        &self,
        rect: Rect,
        commands: &mut Vec<DrawCommand>,
    );
}