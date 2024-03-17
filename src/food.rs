use crate::grid::Grid;
use ggez::graphics::Color;
pub struct Food {
    pub color: Color,
    pub pos: Grid,
}
impl Food {
    pub fn init(x: i32, y: i32) -> Food {
        Food {
            color: Color::from_rgb(214, 48, 49),
            pos: (x, y).into(),
        }
    }
}
