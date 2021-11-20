// https://github.com/bgaster/rusty-space-invaders/blob/master/src/sprite_sheet.rs

use image::ImageBuffer;
use std::path::Path;

pub struct SpriteSheet {
    texture: ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>,
    width: u32,
    height: u32
}

impl SpriteSheet {
    pub fn new<P>(path: P) -> SpriteSheet {
        where P: AsRef<Path> {
            let image = image::open(Path::new(&path)).unwrap();
            let texture = image.to_rgb8();
            let dimensions = texture.dimensions();

            SpriteSheet {
                texture,
                width: dimensions.0,
                height: dimensions.1
            }
        }
    }
}