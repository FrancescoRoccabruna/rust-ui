use crate::{Color, DrawCommand, Rect};

pub trait Renderer {
    type Error;

    fn render(
        &mut self,
        commands: &[DrawCommand],
    ) -> Result<(), Self::Error>;
}


pub struct SoftwareRenderer {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

#[derive(Debug)]
pub struct RenderError;


impl Renderer for SoftwareRenderer {
    type Error = RenderError;

    fn render(
        &mut self,
        commands: &[DrawCommand],
    ) -> Result<(), Self::Error> {
        for command in commands {
            match command {
                DrawCommand::Rect { rect, color } => {
                    self.draw_rect(*rect, *color);
                }

                DrawCommand::Text { .. } => {
                    // per ora dopo
                }
            }
        }

        Ok(())
    }
}

impl SoftwareRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; (width * height * 4) as usize],
        }
    }

    pub fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    pub fn clear(&mut self, color: Color) {
        for pixel in self.pixels.chunks_exact_mut(4) {
            pixel[0] = color.r;
            pixel[1] = color.g;
            pixel[2] = color.b;
            pixel[3] = color.a;
        }
    }


    fn draw_rect(
        &mut self,
        rect: Rect,
        color: Color,
    ) {
        let start_x = rect.origin.x
            .max(0.0)
            .min(self.width as f32) as u32;

        let start_y = rect.origin.y
            .max(0.0)
            .min(self.height as f32) as u32;

        let end_x = (rect.origin.x + rect.size.width)
            .max(0.0)
            .min(self.width as f32) as u32;

        let end_y = (rect.origin.y + rect.size.height)
            .max(0.0)
            .min(self.height as f32) as u32;

        for y in start_y..end_y {
            for x in start_x..end_x {
                let index = ((y * self.width + x) * 4) as usize;

                self.pixels[index] = color.r;
                self.pixels[index + 1] = color.g;
                self.pixels[index + 2] = color.b;
                self.pixels[index + 3] = color.a;
            }
        }
    }

    pub const fn width(&self) -> u32 {
        self.width
    }

    pub const fn height(&self) -> u32 {
        self.height
    }

    pub fn resize(
        &mut self,
        width: u32,
        height: u32,
    ) {
        self.width = width;
        self.height = height;

        self.pixels
            .resize(
                (width * height * 4) as usize,
                0,
            );
    }
}