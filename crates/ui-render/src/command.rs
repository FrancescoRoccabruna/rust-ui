use crate::{Color, Point, Rect};

#[derive(Debug, Clone, PartialEq)]
pub enum DrawCommand {
    Rect {
        rect: Rect,
        color: Color,
    },

    Text {
        position: Point,
        content: String,
        font_size: f32,
    },
}