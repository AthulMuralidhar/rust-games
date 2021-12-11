// https://github.com/bgaster/rusty-space-invaders/blob/master/src/sprite_sheet.rs

use image::{ImageBuffer, DynamicImage};
use std::path::Path;


#[derive(Debug, Clone)]
pub struct SpriteSheet {
    pub texture: ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>,
    pub width: u32,
    pub height: u32
}

impl SpriteSheet {
    pub fn new<P>(image_file: P) -> SpriteSheet
        where P: AsRef<Path> {

            let img : DynamicImage = image::open(image_file).unwrap();
            let tex = img.to_rgba8();
            let dim = tex.dimensions();
            SpriteSheet {
                texture: tex,
                width: dim.0,
                height: dim.1,
            }
    }
}