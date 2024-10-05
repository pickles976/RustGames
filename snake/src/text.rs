extern crate sdl2;

use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, TextureQuery};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::ttf::{Font, Sdl2TtfContext};

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
pub fn get_centered_rect(canvas: &mut WindowCanvas, rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (canvas.viewport().width() as i32 - w) / 2;
    let cy = (canvas.viewport().height() as i32 - h) / 2;
    rect!(cx, cy, w, h)
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
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())?;
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    // let TextureQuery { width, height, .. } = texture.query();

    return Ok(texture);
}