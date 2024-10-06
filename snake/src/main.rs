mod structs;
mod text;
mod colors;

use structs::{GameState, TextMap};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use rand::prelude::*;
use std::collections::VecDeque;
use std::time::Duration;
use std::path::Path;

use crate::structs::{Direction, GameContext, Position, Snake};
use crate::structs::{GRID_SIZE_PX, W, H, INIT_SPEED};
use crate::colors::{PUKE_GREEN, DARK_GREEN};
use crate::text::load_font;

fn draw_square(canvas: &mut WindowCanvas, pos: &Position) -> Result<(), String> {
    
    // Fill rect
    canvas.set_draw_color(DARK_GREEN);
    canvas.fill_rect(rect!(
        (pos.x % W) * GRID_SIZE_PX,
        (pos.y % H) * GRID_SIZE_PX,
        GRID_SIZE_PX as u32,
        GRID_SIZE_PX as u32
    ))?;

    // Separate rects
    canvas.set_draw_color(PUKE_GREEN);
    canvas.draw_rect(rect!(
        (pos.x % W) * GRID_SIZE_PX,
        (pos.y % H) * GRID_SIZE_PX,
        GRID_SIZE_PX as u32,
        GRID_SIZE_PX as u32
    ))?;
    Ok(())
}

fn render_gameplay(canvas: &mut WindowCanvas, game_context: &GameContext) -> Result<(), String> {

    canvas.set_draw_color(PUKE_GREEN);
    canvas.clear();

    for pos in game_context.snake.positions.iter() {
        // draw rect
        draw_square(canvas, pos)?;
    }

    // Draw food
    draw_square(canvas, &game_context.food.position)?;

    canvas.present();

    Ok(())
}

fn render_gameover(canvas: &mut WindowCanvas, text_map: &TextMap) -> Result<(), String> {

    canvas.set_draw_color(PUKE_GREEN);
    canvas.clear();

    canvas.copy(&text_map.game_over_text.texture, None, text_map.game_over_text.rect)?;
    canvas.copy(&text_map.continue_text.texture, None, text_map.continue_text.rect)?;

    canvas.present();

    Ok(())
}

fn update_snake_position(mut snake: Snake) -> Snake {
    // Our snake's head is at the end of the vec
    // [tail, ..., head]
    use crate::structs::Direction::{Down, Left, Right, Up};
    let head_index: usize = snake.positions.len();
    let mut head = snake.positions[head_index - 1].clone();
    match snake.direction {
        Left => {
            head = head.offset(-1, 0);
        }
        Right => {
            head = head.offset(1, 0);
        }
        Up => {
            head = head.offset(0, -1);
        }
        Down => {
            head = head.offset(0, 1);
        }
    }
    snake.positions.pop_front(); // Remove tail
    snake.positions.push_back(head); // Push new head
    snake
}

fn update_game_context(mut game_context: GameContext, mut event_queue: VecDeque<Direction>) -> GameContext {

    // Check if Snake out of bounds
    for pos in game_context.snake.positions.iter() {
        if pos.x < 0 || pos.x == W || pos.y < 0 || pos.y == H {
            game_context.state = GameState::GameOver;
            return game_context;
        }
    }

    // Check if Snake self-intersection
    for outer in 0..game_context.snake.positions.len() - 1 {
        for inner in 0..game_context.snake.positions.len() - 1 {
            if outer == inner {
                continue;
            }

            let pos_outer = game_context.snake.positions.get(outer).unwrap();
            let pos_inner = game_context.snake.positions.get(inner).unwrap();

            if pos_outer.x == pos_inner.x && pos_outer.y == pos_inner.y {
                game_context.state = GameState::GameOver;
                return game_context;
            }
        }
    }

    // Check if food has overlap with snake
    if game_context.snake.positions.contains(&game_context.food.position) {
        // Add new snake box
        game_context.snake.positions.push_front(Position::new(0, 0));
        // Move food
        let mut rng = rand::thread_rng();
        game_context.food.position = Position::new(rng.gen_range(1..W-2), rng.gen_range(1..H-2));
    }

    // Get most recent input event
    match event_queue.pop_back() {
        Some(direction) => {
            // Dont allow the snake to turn in on itself
            match direction {
                Direction::Down => {
                    if game_context.snake.direction != Direction::Up {
                        game_context.snake.direction = direction;
                    }
                },
                Direction::Up => {
                    if game_context.snake.direction != Direction::Down {
                        game_context.snake.direction = direction;
                    }
                },
                Direction::Left => {
                    if game_context.snake.direction != Direction::Right {
                        game_context.snake.direction = direction;
                    }
                },
                Direction::Right => {
                    if game_context.snake.direction != Direction::Left {
                        game_context.snake.direction = direction;
                    }
                }
            }
        },
        _none => {}
    }
    game_context.snake = update_snake_position(game_context.snake);
    game_context.speed = INIT_SPEED + (game_context.snake.positions.len() / 3) as u32; // Increase speed with length
    game_context
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Snake",
            (GRID_SIZE_PX * W) as u32,
            (GRID_SIZE_PX * H) as u32,
        )
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem!");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not make a canvas!");
    let texture_creator = canvas.texture_creator();

    // Load textures with our text in them
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let path: &Path = Path::new("./assets/retro_computer_personal_use.ttf");
    let font = load_font(path, &ttf_context)?;
    let textmap = TextMap::new(&font, &texture_creator)?;

    let mut game_context = GameContext::new();

    let mut event_pump = sdl_context.event_pump()?;

    // 'running is a lifetime annotation
    'running: loop {
        let mut event_queue: VecDeque<Direction> = VecDeque::new();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    // reset game state
                    if game_context.state == GameState::GameOver {
                        game_context = GameContext::new();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    event_queue.push_back(Direction::Left);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    event_queue.push_back(Direction::Right);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    event_queue.push_back(Direction::Up);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    event_queue.push_back(Direction::Down);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                }
                | Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {}
                _ => {}
            }
        }

        // Update
        match game_context.state {
            GameState::Running => {game_context = update_game_context(game_context, event_queue);},
            GameState::GameOver => {}
        }

        // Render
        match game_context.state {
            GameState::Running => {render_gameplay(&mut canvas, &game_context)?;},
            GameState::GameOver => {render_gameover(&mut canvas, &textmap)?;}
        }

        // Time Management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / game_context.speed));
    }
    Ok(())
}
