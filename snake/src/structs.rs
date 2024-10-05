use std::collections::VecDeque;
use rand::prelude::*;

use sdl2::rect::Rect;
use sdl2::render::Texture;

pub const GRID_SIZE_PX: i32 = 32;
pub const H: i32 = 16;
pub const W: i32 = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }

    pub fn offset(&self, x: i32, y: i32) -> Position {
        Position {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Running,
    GameOver
}

#[derive(Debug)]
pub struct Snake {
    pub positions: VecDeque<Position>,
    pub direction: Direction,
}

#[derive(Debug)]
pub struct Food {
    pub position: Position,
}

#[derive(Debug)]
pub struct GameContext {
    pub snake: Snake,
    pub food: Food,
    pub speed: u32,
    pub state: GameState
}

impl GameContext {
    pub fn new() -> GameContext {
        let mut rng = rand::thread_rng();
        GameContext {
            snake: Snake {
                positions: VecDeque::from([
                    Position::new(W / 2, H / 2),
                    Position::new((W / 2) - 1, H / 2),
                    Position::new((W / 2) - 2, H / 2)
                ], 
            ),
                direction: Direction::Right,
            },
            food: Food {
                position: Position::new(rng.gen_range(0..W), rng.gen_range(0..H)),
            },
            speed: 5,
            state: GameState::Running
        }
    }
}

pub struct TextureRect<'a> {
    pub texture: Texture<'a>,
    pub rect: Rect
}

pub struct TextMap<'a> {
    pub game_over_text: TextureRect<'a>,
    pub continue_text: TextureRect<'a>
}
