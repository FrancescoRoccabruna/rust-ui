use ui_render::{DrawCommand, Point, Rect, Size};

use crate::{Constraints, Widget};

pub struct Text {
    content: String,
    font_size: f32,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            font_size: 16.0,
        }
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }
}

impl Widget for Text {
    fn measure(&self, constraints: Constraints) -> Size {
        let estimated_width =
            self.content.len() as f32 * self.font_size * 0.6;

        let estimated_height = self.font_size * 1.2;

        constraints.constrain(
            Size::new(
                estimated_width,
                estimated_height,
            )
        )
    }

    fn draw(
        &self,
        rect: Rect,
        commands: &mut Vec<DrawCommand>,
    ) {
        commands.push(
            DrawCommand::Text {
                position: Point::new(
                    rect.origin.x,
                    rect.origin.y,
                ),
                content: self.content.clone(),
                font_size: self.font_size,
            }
        );
    }
}