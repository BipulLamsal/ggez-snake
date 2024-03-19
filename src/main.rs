use core::f32;

use backgroundgrid::BackgroundgGrid;
use food::Food;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self};
use ggez::graphics::{self, Color};
use ggez::input::keyboard::{self, KeyInput};
use ggez::{Context, GameResult};
use grid::Grid;
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
#[derive(PartialEq, Eq, Clone, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE,
}
impl Direction {
    fn reverse(&self) -> Direction {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::RIGHT => Direction::LEFT,
            Direction::LEFT => Direction::RIGHT,
            _ => Direction::NONE,
        }
    }
}
struct MainGame {
    food: Food,
    snake: Snake,
}
impl MainGame {
    fn init() -> GameResult<MainGame> {
        let mut food = Food::init();
        food.set_random();
        let snake = Snake::new();
        Ok(MainGame { food, snake })
    }
}
impl event::EventHandler<ggez::GameError> for MainGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        while _ctx.time.check_update_time(5) {
            self.snake.update();
            //collision logic
            if self.snake.has_ate_fruit(&self.food) {
                self.food.set_random();
                self.snake.body_list.push(self.snake.head_pos.clone());
            }
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        BackgroundgGrid::draw(ctx, &mut canvas);
        self.food.pos.draw_rect(self.food.color, ctx, &mut canvas);
        self.snake
            .head_pos
            .draw_rect(self.snake.head_color, ctx, &mut canvas);
        let snake_body_iter = self.snake.body_list.clone();
        for item in snake_body_iter.iter() {
            item.draw_rect(self.snake.body_color, ctx, &mut canvas);
        }

        canvas.finish(ctx)
    }
    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        let pressed_key_code = input.keycode.unwrap();
        match pressed_key_code {
            keyboard::KeyCode::W => {
                //conditions rejects if key pressed is same as previous
                //and also rejects moving to reverse direction to current direction
                if self.snake.disallowed_direction() != Direction::UP
                    && self.snake.current_direction != Direction::UP
                {
                    self.snake.last_direction = match self.snake.current_direction.clone() {
                        Direction::NONE => Direction::UP,
                        item => item,
                    };
                    self.snake.current_direction = Direction::UP
                }
            }
            keyboard::KeyCode::S => {
                if self.snake.disallowed_direction() != Direction::DOWN
                    && self.snake.current_direction != Direction::DOWN
                {
                    self.snake.last_direction = match self.snake.current_direction.clone() {
                        Direction::NONE => Direction::DOWN,
                        item => item,
                    };
                    self.snake.current_direction = Direction::DOWN
                }
            }
            keyboard::KeyCode::D => {
                if self.snake.disallowed_direction() != Direction::RIGHT
                    && self.snake.current_direction != Direction::RIGHT
                {
                    self.snake.last_direction = match self.snake.current_direction.clone() {
                        Direction::NONE => Direction::RIGHT,
                        item => item,
                    };
                    self.snake.current_direction = Direction::RIGHT
                }
            }
            keyboard::KeyCode::A => {
                if self.snake.disallowed_direction() != Direction::LEFT
                    && self.snake.current_direction != Direction::LEFT
                {
                    self.snake.last_direction = match self.snake.current_direction.clone() {
                        Direction::NONE => Direction::LEFT,
                        item => item,
                    };
                    self.snake.current_direction = Direction::LEFT
                }
            }
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
