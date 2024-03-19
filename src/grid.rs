use super::GRID_SIZE;
use ggez::{
    graphics::{self, Canvas, Color, DrawMode, FillOptions},
    Context,
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid {
    x: i32,
    y: i32,
}
impl Grid {
    pub fn new(x: i32, y: i32) -> Grid {
        Grid { x, y }
    }
    pub fn draw_rect(&self, color: Color, ctx: &mut Context, canvas: &mut Canvas) {
        let mut binding = graphics::MeshBuilder::new();
        let _grid_builder = binding.rectangle(
            DrawMode::Fill(FillOptions::default()),
            graphics::Rect {
                x: self.x as f32,
                y: self.y as f32,
                w: GRID_SIZE.0 as f32,
                h: GRID_SIZE.1 as f32,
            },
            color,
        );
        let grid = graphics::Mesh::from_data(ctx, binding.build());
        canvas.draw(&grid, graphics::DrawParam::default());
    }
    pub fn modify(&mut self, new_x: i32, new_y: i32) {
        self.x = new_x;
        self.y = new_y;
    }
    pub fn get_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}
impl From<(i32, i32)> for Grid {
    fn from(pos: (i32, i32)) -> Self {
        Grid { x: pos.0, y: pos.1 }
    }
}
impl Into<(i32, i32)> for Grid {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}
