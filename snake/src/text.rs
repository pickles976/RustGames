extern crate sdl2;

use std::path::Path;
use crate::colors::{DARK_GREEN};
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas, TextureCreator};
use sdl2::ttf::{Font, Sdl2TtfContext};

// handle the annoying Rect i32
#[macro_export]
macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr) => {
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    };
}

pub fn load_font<'a>(font_path: &'a Path, ttf_context: &'a Sdl2TtfContext) -> Result<Font<'a, 'a>, String> {

    // Load a font
    let mut font = ttf_context.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    return Ok(font);
}

pub fn create_text_texture<'a, T>(font: &Font, text: String, texture_creator: &'a TextureCreator<T>) -> Result<Texture<'a>, String> {
        // render a surface, and convert it to a texture bound to the canvas
        let surface = font
        .render(&text)
        .blended(DARK_GREEN)
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    return Ok(texture);
}