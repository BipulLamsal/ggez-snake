use crate::{grid::Grid, DIMENSION_SIZE, GRID_SIZE};
use ggez::graphics::Color;
use rand::Rng;
#[derive(Debug)]
pub struct Food {
    pub color: Color,
    pub pos: Grid,
}
impl Food {
    pub fn set_random(&mut self) {
        let mut rng = rand::thread_rng();
        let rand_pos_x = rng.gen_range(0..DIMENSION_SIZE.0) as i32 * GRID_SIZE.0;
        let rand_pos_y = rng.gen_range(0..DIMENSION_SIZE.1) as i32 * GRID_SIZE.1;
        self.pos = (rand_pos_x, rand_pos_y).into();
    }
    pub fn init() -> Food {
        Food {
            color: Color::from_rgb(214, 48, 49),
            pos: (0, 0).into(),
        }
    }
}
