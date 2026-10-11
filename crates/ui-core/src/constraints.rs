use ui_render::Size;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Constraints {
    pub min_size: Size,
    pub max_size: Size,
}

impl Constraints {
    pub const fn new(
        min_size: Size,
        max_size: Size,
    ) -> Self {
        Self {
            min_size,
            max_size,
        }
    }

    pub const fn tight(size: Size) -> Self {
        Self {
            min_size: size,
            max_size: size,
        }
    }

    pub const fn loose(max_size: Size) -> Self {
        Self {
            min_size: Size::new(0.0, 0.0),
            max_size,
        }
    }

    pub fn constrain(&self, size: Size) -> Size {
        Size::new(
            size.width.clamp(self.min_size.width, self.max_size.width),
            size.height.clamp(self.min_size.height, self.max_size.height),
        )
    }
}