use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use sdl2::sys::KeyCode;
use std::time::Duration;

#[derive(Debug)]
struct Player {
    position: Point, 
    sprite: Rect,
    speed: i32
}

fn render(canvas: &mut WindowCanvas, color: Color, texture: &Texture, player: &Player) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();
    let (width, height) = canvas.output_size()?;

    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());

    canvas.copy(texture, player.sprite, screen_rect)?;
    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem!");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not make a canvas!");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/bardo.png")?;

    let mut player = Player { 
        position: Point::new(0,0), 
        sprite: Rect::new(0,0,26,36),
        speed: 5
    };

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    // 'running is a lifetime annotation
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { 
                    break 'running; 
                },
                Event::KeyDown { keycode: Some(Keycode::Left),  .. } => {
                    player.position = player.position.offset(-player.speed, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Right),  .. } => {
                    player.position = player.position.offset(player.speed, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Up),  .. } => {
                    player.position = player.position.offset(0, -player.speed);
                },
                Event::KeyDown { keycode: Some(Keycode::Down),  .. } => {
                    player.position = player.position.offset(0, player.speed);
                },
                _ => {}
            }
        }

        // Update
        i = (i + 1) % 255;

        // Render
        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, &player)?;

        // Time Management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}
