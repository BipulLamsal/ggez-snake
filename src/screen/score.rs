use crate::{GRID_SIZE, SCREEN_SIZE};
use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color, DrawMode, FillOptions, PxScale, Text, TextFragment},
    Context,
};
pub struct Score {
    pub value: i32,
}
impl Score {
    pub fn init() -> Score {
        Score { value: 0 }
    }
    pub fn draw_screen(&self, ctx: &mut Context, canvas: &mut Canvas) {
        let mut binding = graphics::MeshBuilder::new();
        let _grid_builder = binding.rectangle(
            DrawMode::Fill(FillOptions::default()),
            graphics::Rect {
                x: GRID_SIZE.0 as f32 * 2.0,
                y: GRID_SIZE.0 as f32 * 7.0,
                w: (SCREEN_SIZE.0 - GRID_SIZE.0 as f32 * 4.0),
                h: GRID_SIZE.1 as f32 * 4.0 as f32,
            },
            Color::from_rgb(245, 246, 250),
        );

        let text_score = "Gameover! Your score is ".to_string()
            + &self.value.to_string()
            + "\n Press R to start again";
        let grid = graphics::Mesh::from_data(ctx, binding.build());
        let heading_text = Text::new(TextFragment {
            text: text_score,
            color: Some(Color::from_rgb(46, 204, 113)),
            font: Some("LiberationMono-Regular".into()),
            scale: Some(PxScale::from(SCREEN_SIZE.0 / 25.0)),
            ..Default::default()
        });

        canvas.draw(&grid, graphics::DrawParam::default());
        canvas.draw(
            &heading_text,
            Vec2::new(GRID_SIZE.0 as f32 * 3.0, GRID_SIZE.1 as f32 * 8.0),
        );
    }
}
