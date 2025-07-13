use image::{ImageBuffer, Rgb};
use crate::polygon::Color;

pub struct Framebuffer {
    pub buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
    pub width: u32,
    pub height: u32,
    pub current_color: Color,
    pub background_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Framebuffer {
            buffer: ImageBuffer::new(width, height),
            width,
            height,
            current_color: Color::new(0, 0, 0),
            background_color: Color::new(255, 255, 255),
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.buffer.put_pixel(
                x as u32, 
                y as u32, 
                Rgb([self.current_color.r, self.current_color.g, self.current_color.b])
            );
        }
    }

    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.buffer.put_pixel(
                    x, 
                    y, 
                    Rgb([self.background_color.r, self.background_color.g, self.background_color.b])
                );
            }
        }
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn save(&self, filename: &str) -> Result<(), image::ImageError> {
        self.buffer.save(filename)
    }
}