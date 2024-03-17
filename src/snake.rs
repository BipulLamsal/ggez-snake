use ggez::graphics::Color;

use crate::{grid::Grid, Direction, GRID_SIZE};

pub struct Snake {
    pub head_pos: Grid,
    pub head_color: Color,
    pub last_direction: Direction,
}
impl Snake {
    pub fn new() -> Snake {
        Snake {
            head_pos: (32, 32).into(),
            head_color: Color::from_rgb(9, 132, 227),
            last_direction: Direction::NONE,
        }
    }
    pub fn update(&mut self) {
        let prev_head_position: (i32, i32) = self.head_pos.clone().into();
        match self.last_direction {
            Direction::UP => {
                self.head_pos = (prev_head_position.0, prev_head_position.1 - GRID_SIZE.1).into();
            }
            Direction::DOWN => {
                self.head_pos = (prev_head_position.0, prev_head_position.1 + GRID_SIZE.1).into();
            }
            Direction::LEFT => {
                self.head_pos = (prev_head_position.0 - GRID_SIZE.0, prev_head_position.1).into();
            }
            Direction::RIGHT => {
                self.head_pos = (prev_head_position.0 + GRID_SIZE.0, prev_head_position.1).into();
            }
            _ => {}
        }
    }
}
