use ui_render::{DrawCommand, Rect, Size};

use crate::{Constraints, Widget};

pub struct Column {
    children: Vec<Box<dyn Widget>>,
    spacing: f32,
}

impl Column {
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

impl Default for Column {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Column {
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

            let remaining_height =
                (constraints.max_size.height - height - spacing).max(0.0);

            let child_size = child.measure(
                Constraints::loose(Size::new(
                    constraints.max_size.width,
                    remaining_height,
                ))
            );

            width = width.max(child_size.width);
            height += spacing + child_size.height;
        }

        constraints.constrain(Size::new(width, height))
    }

    fn draw(
        &self,
        rect: Rect,
        commands: &mut Vec<DrawCommand>,
    ) {
        let mut y = rect.origin.y;
        let bottom = rect.origin.y + rect.size.height;

        for child in &self.children {
            let remaining_height = (bottom - y).max(0.0);

            let child_size = child.measure(
                Constraints::loose(Size::new(
                    rect.size.width,
                    remaining_height,
                ))
            );

            let child_rect = Rect::new(
                rect.origin.x,
                y,
                child_size.width,
                child_size.height,
            );

            child.draw(child_rect, commands);

            y += child_size.height + self.spacing;
        }
    }
}