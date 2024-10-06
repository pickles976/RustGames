extern crate sdl2;

use std::path::Path;
use sdl2::rect::Rect;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::render::{Texture, TextureCreator};

use crate::colors::DARK_GREEN;
use crate::structs::GRID_SIZE_PX;



// handle the annoying Rect i32
#[macro_export]
macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr) => {
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    };
}

pub fn get_rect_for_lines(start_column: i32, start_row: i32, end_column: i32, end_row: i32) -> Rect {

    let width : i32 = (end_column - start_column) * GRID_SIZE_PX;
    let height : i32 = (end_row - start_row) * GRID_SIZE_PX;

    rect!(
        start_column * GRID_SIZE_PX,
        start_row * GRID_SIZE_PX,
        width,
        height
    )
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