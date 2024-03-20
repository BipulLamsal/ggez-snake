use crate::{GRID_SIZE, SCREEN_SIZE};
use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, Color, DrawMode, FillOptions, PxScale, Text, TextFragment},
    Context,
};

pub struct Menu {
    button_x: f32,
    button_y: f32,
    button_length: f32,
    button_height: f32,
}
impl Menu {
    pub fn init() -> Menu {
        Menu {
            button_x: GRID_SIZE.0 as f32 * 4.0,
            button_y: GRID_SIZE.1 as f32 * 10.0,
            button_length: SCREEN_SIZE.0 - GRID_SIZE.0 as f32 * 8.0,
            button_height: GRID_SIZE.1 as f32 * 3.0 as f32,
        }
    }
    pub fn get_button_range_x(&self) -> (f32, f32) {
        (self.button_x, self.button_x + self.button_length)
    }
    pub fn get_button_range_y(&self) -> (f32, f32) {
        (self.button_y, self.button_y + self.button_height)
    }
    pub fn draw_screen(&self, ctx: &mut Context, canvas: &mut Canvas) {
        let mut binding = graphics::MeshBuilder::new();
        let mut button_binding = graphics::MeshBuilder::new();
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
        let _button_builder = button_binding.rectangle(
            DrawMode::Fill(FillOptions::default()),
            graphics::Rect {
                x: self.button_x,
                y: self.button_y,
                w: self.button_length,
                h: self.button_height,
            },
            Color::from_rgb(53, 59, 72),
        );

        let grid = graphics::Mesh::from_data(ctx, binding.build());
        let button = graphics::Mesh::from_data(ctx, button_binding.build());
        let heading_text = Text::new(TextFragment {
            text: "ggez sabinonweb snake".to_string(),
            color: Some(Color::from_rgb(53, 59, 72)),
            font: Some("LiberationMono-Regular".into()),
            scale: Some(PxScale::from(SCREEN_SIZE.0 / 17.0)),
            ..Default::default()
        });
        let button_text = Text::new(TextFragment {
            text: "start (Press/Enter)".to_string(),
            color: Some(Color::from_rgb(255, 255, 255)),
            font: Some("LiberationMono-Regular".into()),
            scale: Some(PxScale::from(SCREEN_SIZE.0 / 20.0)),
            ..Default::default()
        });
        canvas.draw(&grid, graphics::DrawParam::default());
        canvas.draw(&button, graphics::DrawParam::default());
        canvas.draw(
            &heading_text,
            Vec2::new(GRID_SIZE.0 as f32 * 3.0, GRID_SIZE.1 as f32 * 8.0),
        );
        canvas.draw(
            &button_text,
            Vec2::new(
                self.button_x + GRID_SIZE.0 as f32,
                self.button_y + GRID_SIZE.1 as f32,
            ),
        )
    }
}
