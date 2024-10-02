mod dataclasses;

use dataclasses::GameState;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas};
use sdl2::video::{Window, WindowContext};

use rand::prelude::*;
use std::collections::VecDeque;
use std::time::Duration;

use crate::dataclasses::{Direction, Food, GameContext, Position, Snake};
use crate::dataclasses::{GRID_SIZE_PX, W, H};

// fn draw_square<R: Into<Option<Color>>>(canvas: &mut WindowCanvas, pos: &Position, color: R) -> Result<(), String> {
fn draw_square(canvas: &mut WindowCanvas, pos: &Position) -> Result<(), String> {
    // match color.into() {
    //     Some(_color) => { canvas.set_draw_color(_color);},
    //     None => {}
    // }
    
    // draw rect
    canvas.set_draw_color(Color::RGB(111, 97, 0));
    canvas.fill_rect(Rect::new(
        ((pos.x % W) * GRID_SIZE_PX) as i32,
        ((pos.y % H) * GRID_SIZE_PX) as i32,
        GRID_SIZE_PX as u32,
        GRID_SIZE_PX as u32,
    ))?;

    canvas.set_draw_color(Color::RGB(184, 196, 2));
    canvas.draw_rect(Rect::new(
        ((pos.x % W) * GRID_SIZE_PX) as i32,
        ((pos.y % H) * GRID_SIZE_PX) as i32,
        GRID_SIZE_PX as u32,
        GRID_SIZE_PX as u32,
    ))?;
    Ok(())
}

fn render(canvas: &mut WindowCanvas, game_context: &GameContext) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(184, 196, 2));
    canvas.clear();

    // Draw each square in the snake at index
    for pos in game_context.snake.positions.iter() {
        // draw rect
        draw_square(canvas, pos)?;
    }

    // Draw food
    draw_square(canvas, &game_context.food.position)?;

    if game_context.state == GameState::GameOver {
        
    }

    canvas.present();

    Ok(())
}

fn update_snake_position(mut snake: Snake) -> Snake {
    // Our snake's head is at the end of the vec
    // [tail, ..., head]
    use crate::dataclasses::Direction::{Down, Left, Right, Up};
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
    snake.positions.pop_front();
    snake.positions.push_back(head);
    snake
}

fn update_game_state(mut game_context: GameContext, mut event_queue: VecDeque<Direction>) -> GameContext {

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
        game_context.snake.positions.push_front(Position::new(0, 0));
        let mut rng = rand::thread_rng();
        game_context.food.position = Position::new(rng.gen_range(1..W-1), rng.gen_range(1..H-1));
    }

    // Get most recent input event
    match event_queue.pop_back() {
        Some(direction) => {
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
        None => {}
    }
    game_context.snake = update_snake_position(game_context.snake);
    game_context.speed = 5 + (game_context.snake.positions.len() / 3) as u32;
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
        game_context = update_game_state(game_context, event_queue);

        // Render
        render(&mut canvas, &game_context)?;

        // Time Management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / game_context.speed));
    }
    Ok(())
}
