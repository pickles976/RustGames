use std::collections::VecDeque;

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
pub struct GameState {
    pub snake: Snake,
    pub food: Food,
    pub speed: u32,
}
