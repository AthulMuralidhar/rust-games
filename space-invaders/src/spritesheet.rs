extern crate image;
use image::{ImageBuffer, Rgba};

pub struct SpriteSheet {
    pub texture: ImageBuffer<Rgba<u8>, std::vec::Vec<u8>>,
    pub width: u32,
    pub height: u32
}