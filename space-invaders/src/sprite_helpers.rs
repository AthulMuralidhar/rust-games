// https://github.com/bgaster/rusty-space-invaders/blob/master/src/sprite_sheet.rs

use image::{ImageBuffer, DynamicImage};
use std::path::Path;
use std::fs::File;
use serde_derive::Deserialize;
use std::collections::HashMap;
use crate::frame::Frame;

#[derive(Deserialize)]
pub struct XYWHJSON {
    pub x: i32,
    pub y: i32,
    pub h: i32,
    pub w: i32,
}

#[derive(Deserialize)]
pub struct WHJSON {
    pub h: i32,
    pub w: i32,
}

#[derive(Deserialize)]
pub struct MetaJSON {
    pub app: String,
    pub version: String,
    pub image: String,
    pub format: String,
    pub size: WHJSON,
}

#[derive(Deserialize)]
pub struct SpriteJSON {
    pub frame: XYWHJSON,
    pub rotated: bool,
    pub trimmed: bool,
    pub spriteSourceSize: XYWHJSON,
    pub sourceSize: WHJSON,
}

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

#[derive(Clone)]
pub struct Sprite {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
impl Sprite {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self{
        Sprite{x,y,width,height}
    }
    pub fn render<'a>(&self, x: u32, y: u32, sheet: &SpriteSheet, frame: &mut Frame<'a>) {

    }
}

#[derive(Deserialize)]
pub struct SheetJSON {
    pub frames: HashMap<String, SpriteJSON>,
    pub meta: MetaJSON,

}
impl SheetJSON{
    pub fn new<P>(jsonfile: P) -> Self 
        where P: AsRef<Path> {
            let json = String::new();
            let file = File::open(jsonfile).expect("unable to open sheet json file");
            serde_json::from_reader(file).expect("invalid sprite sheet json file")
        }
    
}

