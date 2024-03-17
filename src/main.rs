use core::f32;

use backgroundgrid::BackgroundgGrid;
use food::Food;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color, DrawMode, FillOptions};
use ggez::input::keyboard::{self, KeyInput};
use ggez::{input, Context, GameResult};
use rand::Rng;
use snake::Snake;

pub const DIMENSION_SIZE: (i32, i32) = (20, 20);
pub const GRID_SIZE: (i32, i32) = (32, 32);
pub const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * DIMENSION_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * DIMENSION_SIZE.1 as f32,
);
mod backgroundgrid;
mod food;
mod grid;
mod snake;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE,
}

struct MainGame {
    food: Food,
    snake: Snake,
}
impl MainGame {
    fn init() -> GameResult<MainGame> {
        let food = Food::init(0, 0);
        let snake = Snake::new();
        Ok(MainGame { food, snake })
    }
}
impl event::EventHandler<ggez::GameError> for MainGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.snake.update();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        BackgroundgGrid::draw(ctx, &mut canvas);
        self.food.pos.draw_rect(self.food.color, ctx, &mut canvas);
        self.snake
            .head_pos
            .draw_rect(self.snake.head_color, ctx, &mut canvas);

        canvas.finish(ctx)
    }
    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        let pressed_key_code = input.keycode.unwrap();
        match pressed_key_code {
            keyboard::KeyCode::W => self.snake.last_direction = Direction::UP,
            keyboard::KeyCode::S => self.snake.last_direction = Direction::DOWN,
            keyboard::KeyCode::D => self.snake.last_direction = Direction::RIGHT,
            keyboard::KeyCode::A => self.snake.last_direction = Direction::LEFT,
            _ => println!("Fuck Sabinonweb"),
        }

        Ok(())
    }
}

impl MainGame {}
pub fn main() -> GameResult {
    let window_setup = WindowSetup::default().title("Fucking around here and there");
    let window_mode = WindowMode::default()
        .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1)
        .transparent(true)
        .resizable(false);
    let cb = ggez::ContextBuilder::new("grid", "fucking around")
        .window_setup(window_setup)
        .window_mode(window_mode);
    let (ctx, event_loop) = cb.build()?;
    let game_state = MainGame::init()?;
    event::run(ctx, event_loop, game_state)
}
