use ui_render::{Color, DrawCommand, Rect, Size};

use crate::{Widget, Constraints};

pub struct Rectangle {
    size: Size,
    color: Color,
}

impl Rectangle {
    pub fn new(size: Size, color: Color) -> Self {
        Self { size, color }
    }
}

impl Widget for Rectangle {
    fn measure(&self, constraints: Constraints) -> Size {
        constraints.constrain(self.size)
    }

    fn draw(
        &self,
        rect: Rect,
        commands: &mut Vec<DrawCommand>,
    ) {
        commands.push(DrawCommand::Rect {
            rect,
            color: self.color,
        });
    }
}