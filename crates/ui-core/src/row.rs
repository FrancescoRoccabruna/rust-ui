use ui_render::{DrawCommand, Rect, Size};

use crate::{Constraints, Widget};

pub struct Row {
    children: Vec<Box<dyn Widget>>,
    spacing: f32,
}

impl Row {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: 0.0,
        }
    }

    pub fn child<W>(mut self, widget: W) -> Self
    where
        W: Widget + 'static,
    {
        self.children.push(Box::new(widget));
        self
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Row {
    fn measure(&self, constraints: Constraints) -> Size {
        if self.children.is_empty() {
            return constraints.constrain(Size::new(0.0, 0.0));
        }

        let mut width: f32 = 0.0;
        let mut height: f32 = 0.0;

        for (index, child) in self.children.iter().enumerate() {
            let spacing = if index == 0 {
                0.0
            } else {
                self.spacing
            };

            let remaining_width =
                (constraints.max_size.width - width - spacing).max(0.0);

            let child_size = child.measure(
                Constraints::loose(Size::new(
                    remaining_width,
                    constraints.max_size.height,
                ))
            );

            height = height.max(child_size.height);
            width += spacing + child_size.width;
        }

        constraints.constrain(Size::new(width, height))
    }

    fn draw(
        &self,
        rect: Rect,
        commands: &mut Vec<DrawCommand>,
    ) {
        let mut x = rect.origin.x;
        let right = rect.origin.x + rect.size.width;

        for child in &self.children {
            let remaining_width = (right - x).max(0.0);

            let child_size = child.measure(
                Constraints::loose(Size::new(
                    remaining_width,
                    rect.size.height,
                ))
            );

            let child_rect = Rect::new(
                x,
                rect.origin.y,
                child_size.width,
                child_size.height,
            );

            child.draw(child_rect, commands);

            x += child_size.width + self.spacing;
        }
    }
}