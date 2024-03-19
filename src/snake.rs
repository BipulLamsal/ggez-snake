use crate::{food::Food, grid::Grid, Direction, GRID_SIZE, SCREEN_SIZE};
use ggez::graphics::Color;
#[derive(Debug)]
pub struct Snake {
    pub head_pos: Grid,
    pub head_color: Color,
    pub body_color: Color,
    pub body_list: Vec<Grid>,
    pub current_direction: Direction,
    pub last_direction: Direction,
}
impl Snake {
    pub fn new() -> Snake {
        Snake {
            head_pos: (32, 32).into(),
            head_color: Color::from_rgb(9, 132, 227),
            body_color: Color::from_rgb(116, 185, 255),
            body_list: Vec::new(),
            last_direction: Direction::NONE,
            current_direction: Direction::NONE,
        }
    }
    pub fn update(&mut self) {
        let mut prev_head_position: (i32, i32) = self.head_pos.clone().into();
        let mut prev_segment_position: (i32, i32) = self.head_pos.clone().into();
        //head movement
        match self.current_direction {
            Direction::UP => {
                if prev_head_position.1 <= 0 {
                    prev_head_position.1 = SCREEN_SIZE.1 as i32;
                }
                self.head_pos = (prev_head_position.0, prev_head_position.1 - GRID_SIZE.1).into();
            }
            Direction::DOWN => {
                if prev_head_position.1 >= SCREEN_SIZE.1 as i32 {
                    prev_head_position.1 = GRID_SIZE.1 as i32 - GRID_SIZE.1 as i32 * 2;
                }
                self.head_pos = (prev_head_position.0, prev_head_position.1 + GRID_SIZE.1).into();
            }
            Direction::LEFT => {
                if prev_head_position.0 <= 0 {
                    prev_head_position.0 = SCREEN_SIZE.0 as i32;
                }
                self.head_pos = (prev_head_position.0 - GRID_SIZE.0, prev_head_position.1).into();
            }
            Direction::RIGHT => {
                if prev_head_position.0 >= SCREEN_SIZE.0 as i32 {
                    prev_head_position.0 = GRID_SIZE.1 as i32 - GRID_SIZE.1 as i32 * 2;
                }
                self.head_pos = (prev_head_position.0 + GRID_SIZE.0, prev_head_position.1).into();
            }
            _ => {}
        }

        //exhanges the segment positions
        for segment in &mut self.body_list {
            let curret_position: (i32, i32) = segment.get_tuple();
            segment.modify(prev_segment_position.0, prev_segment_position.1);
            prev_segment_position = curret_position;
        }
    }
    pub fn has_ate_fruit(&self, food: &Food) -> bool {
        let food_pos = &food.pos;
        if *food_pos == self.head_pos {
            return true;
        }
        false
    }

    pub fn disallowed_direction(&self) -> Direction {
        self.current_direction.reverse()
    }
}
