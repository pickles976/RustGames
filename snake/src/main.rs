use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas, TextureCreator, Canvas};
use sdl2::video::{WindowContext, Window};

use std::time::Duration;
use std::collections::VecDeque;
use rand::prelude::*;

const GRID_SIZE_PX: i32 = 32;
const H: i32 = 32;
const W: i32 = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { 
            x: x,
            y: y
        }
    }

    pub fn offset(&self, x: i32, y: i32) -> Position {
        Position {
            x: self.x + x,
            y: self.y + y
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Snake {
    positions: VecDeque<Position>, 
    direction: Direction,
}

#[derive(Debug)]
struct Food {
    position: Position
}

#[derive(Debug)]
struct GameState {
    snake: Snake,
    food: Food,
    speed: u32
}

fn dummy_texture<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Texture<'a>, String> {
    enum TextureColor {
        White,
    }
    let mut square_texture = texture_creator
        .create_texture_target(None, GRID_SIZE_PX as u32, GRID_SIZE_PX as u32)
        .map_err(|e| e.to_string())?;
    // let's change the textures we just created
    {
        let textures = [
            (&mut square_texture, TextureColor::White),
        ];
        canvas
            .with_multiple_texture_canvas(textures.iter(), |texture_canvas, user_context| {
                texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                texture_canvas.clear();
                match *user_context {
                    TextureColor::White => {
                        for i in 0..GRID_SIZE_PX {
                            for j in 0..GRID_SIZE_PX {
                                // drawing pixel by pixel isn't very effective, but we only do it once and store
                                // the texture afterwards so it's still alright!
                                if (i + j) % 7 == 0 {
                                    // this doesn't mean anything, there was some trial and error to find
                                    // something that wasn't too ugly
                                    texture_canvas.set_draw_color(Color::RGB(192, 192, 192));
                                    texture_canvas
                                        .draw_point(Point::new(i as i32, j as i32))
                                        .expect("could not draw point");
                                }
                                if (i + j * 2) % 5 == 0 {
                                    texture_canvas.set_draw_color(Color::RGB(64, 64, 64));
                                    texture_canvas
                                        .draw_point(Point::new(i as i32, j as i32))
                                        .expect("could not draw point");
                                }
                            }
                        }
                    }
                };
                for i in 0..GRID_SIZE_PX {
                    for j in 0..GRID_SIZE_PX {
                        // drawing pixel by pixel isn't very effective, but we only do it once and store
                        // the texture afterwards so it's still alright!
                        texture_canvas.set_draw_color(Color::RGB(192, 192, 192));
                        texture_canvas
                            .draw_point(Point::new(i as i32, j as i32))
                            .expect("could not draw point");
                    }
                }
            })
            .map_err(|e| e.to_string())?;
    }
    Ok(square_texture)
}

fn draw_square(canvas: &mut WindowCanvas, texture: &Texture, pos: &Position) -> Result<(), String> {
    // draw rect
    canvas.copy(
        texture,
        None,
        Rect::new(
            ((pos.x % W) * GRID_SIZE_PX) as i32,
            ((pos.y % H) * GRID_SIZE_PX) as i32,
            GRID_SIZE_PX as u32,
            GRID_SIZE_PX as u32,
        ),
    )?;
    Ok(())
}

fn render(canvas: &mut WindowCanvas, color: Color, texture: &Texture, gamestate: &GameState) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    // Draw each square in the snake at index
    for pos in gamestate.snake.positions.iter() {    
        // draw rect
        draw_square(canvas, texture, pos)?;
    }

    // Draw food
    draw_square(canvas, texture, &gamestate.food.position)?;


    canvas.present();

    Ok(())
}

fn update_snake(mut snake: Snake) -> Snake {
    // Our snake's head is at the end of the vec
    // [tail, ..., head]
    use self::Direction::*;
    let head_index: usize = snake.positions.len();
    let mut head = snake.positions[head_index - 1].clone();
    match snake.direction {
        Left => {
            head = head.offset(-1, 0);
        },
        Right => {
            head = head.offset(1, 0);
        },
        Up => {
            head = head.offset(0, -1);
        },
        Down => {
            head = head.offset(0, 1);
        },
    }
    snake.positions.pop_front();
    snake.positions.push_back(head);
    snake
}


fn update_game_state(mut gamestate: GameState, mut event_queue: VecDeque<Direction>) -> GameState {

    // Check if food has overlap with snake
    if gamestate.snake.positions.contains(&gamestate.food.position) {
        gamestate.snake.positions.push_front(Position::new(0,0));
        let mut rng = rand::thread_rng();
        gamestate.food.position = Position::new(
            rng.gen_range(0..W),
            rng.gen_range(0..H)
        );
    }

    // Get most recent input event
    match event_queue.pop_back() {
        Some(direction) => gamestate.snake.direction = direction,
        None => {}
    }
    gamestate.snake = update_snake(gamestate.snake);
    gamestate.speed = 5 + (gamestate.snake.positions.len() / 3) as u32;
    gamestate
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Snake", (GRID_SIZE_PX * W) as u32, (GRID_SIZE_PX * H) as u32)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem!");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not make a canvas!");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let square_texture = dummy_texture(&mut canvas, &texture_creator)?;

    let mut rng = rand::thread_rng();
    let mut gamestate = GameState {
        snake: Snake { 
            positions: VecDeque::from([Position::new(W / 2, H / 2)]), 
            direction: Direction::Right,
        },
        food: Food {
            position: Position::new(
                rng.gen_range(0..W),
                rng.gen_range(0..H)
            )
        },
        speed: 5
    };

    let mut event_pump = sdl_context.event_pump()?;

    // 'running is a lifetime annotation
    'running: loop {
        let mut event_queue : VecDeque<Direction> = VecDeque::new();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { 
                    break 'running; 
                },
                Event::KeyDown { keycode: Some(Keycode::Left),  .. } => {
                    event_queue.push_back(Direction::Left);
                },
                Event::KeyDown { keycode: Some(Keycode::Right),  .. } => {
                    event_queue.push_back(Direction::Right);
                },
                Event::KeyDown { keycode: Some(Keycode::Up),  .. } => {
                    event_queue.push_back(Direction::Up);
                },
                Event::KeyDown { keycode: Some(Keycode::Down),  .. } => {
                    event_queue.push_back(Direction::Down);
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {},
                _ => {}
            }
        }

        // Update
        gamestate = update_game_state(gamestate, event_queue);

        // Render
        render(&mut canvas, Color::RGB(0, 0, 0), &square_texture, &gamestate)?;

        // Time Management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / gamestate.speed));
    }
    Ok(())
}
