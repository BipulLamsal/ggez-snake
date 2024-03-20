use crate::{grid::Grid, snake::Snake, DIMENSION_SIZE, GRID_SIZE};
use ggez::graphics::Color;
use rand::seq::SliceRandom;
#[derive(Debug)]
pub struct Food {
    pub color: Color,
    pub pos: Grid,
}
impl Food {
    pub fn set_random(&mut self, snake: &Snake) {
        let mut rng = rand::thread_rng();
        let mut possible_positions = Vec::new();
        for x in 0..DIMENSION_SIZE.0 {
            for y in 0..DIMENSION_SIZE.1 {
                let pos = ((x as i32) * GRID_SIZE.0, (y as i32) * GRID_SIZE.1);
                if !snake.body_list.iter().any(|item| item.get_tuple() == pos) {
                    possible_positions.push(pos);
                }
            }
        }
        if let Some(rand_pos) = possible_positions.choose(&mut rng) {
            self.pos = (rand_pos.0, rand_pos.1).into()
        }
    }

    pub fn init() -> Food {
        Food {
            color: Color::from_rgb(214, 48, 49),
            pos: (0, 0).into(),
        }
    }
}
