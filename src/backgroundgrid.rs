use ggez::graphics::Canvas;
use ggez::graphics::Color;
use ggez::Context;

use super::GRID_SIZE;
use super::SCREEN_SIZE;
use crate::grid::Grid;
pub struct BackgroundgGrid;
impl BackgroundgGrid {
    pub fn draw(ctx: &mut Context, canvas: &mut Canvas) {
        for i in (0..SCREEN_SIZE.0 as i32 + 1).step_by(32) {
            for j in (0..SCREEN_SIZE.1 as i32 + 1).step_by(32) {
                let grid = Grid::new(i, j);
                if ((i + j) / GRID_SIZE.0 as i32) % 2 == 0 {
                    grid.draw_rect(Color::from_rgb(196, 229, 56), ctx, canvas)
                } else {
                    grid.draw_rect(Color::from_rgb(163, 203, 56), ctx, canvas)
                }
            }
        }
    }
}
