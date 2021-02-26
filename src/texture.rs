use std::io::Cursor;

use glium::texture::Texture2d;
use glium::{Display, texture::RawImage2d};

pub fn load_png_texture(display: &Display, bytes: &[u8]) -> Texture2d {
    let image = image::load(
        Cursor::new(bytes),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions();
    let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(display, image).unwrap();
    texture
}
