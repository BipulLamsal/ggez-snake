use core::f32;

use backgroundgrid::BackgroundgGrid;
use food::Food;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self};
use ggez::graphics::{self, Color};
use ggez::input::keyboard::{self, KeyInput};
use ggez::{Context, GameResult};
use screen::menu::Menu;
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
mod screen;
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

enum Ate {
    Itself,
    Food,
    Nothing,
}

#[derive(PartialEq, Eq)]
enum Screen {
    Menu,
    Game,
    Score,
}

struct MainGame {
    food: Food,
    snake: Snake,
    gameover: bool,
    menu: Menu,
    screen: Screen,
}

impl MainGame {
    fn init() -> GameResult<MainGame> {
        let menu = Menu::init();
        let mut food = Food::init();
        food.set_random();
        let snake = Snake::new();
        Ok(MainGame {
            food,
            snake,
            menu,
            gameover: false,
            screen: Screen::Menu,
        })
    }
}
impl event::EventHandler<ggez::GameError> for MainGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        while _ctx.time.check_update_time(5) {
            self.snake.update();
            //collision logic
            match self.snake.has_ate(&self.food) {
                Ate::Food => {
                    self.food.set_random();
                    let last_position = self.snake.body_list.last().unwrap_or(&self.snake.head_pos);
                    self.snake.body_list.push(last_position.clone());
                }
                Ate::Itself => {
                    self.gameover = true;
                }

                Ate::Nothing => {}
            }
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        BackgroundgGrid::draw(ctx, &mut canvas);
        match self.screen {
            Screen::Menu => self.menu.draw_screen(ctx, &mut canvas),
            Screen::Game => {
                self.food.pos.draw_rect(self.food.color, ctx, &mut canvas);
                if !self.gameover {
                    self.snake
                        .head_pos
                        .draw_rect(self.snake.head_color, ctx, &mut canvas);
                    let snake_body_iter = self.snake.body_list.clone();
                    for item in snake_body_iter.iter() {
                        item.draw_rect(self.snake.body_color, ctx, &mut canvas);
                    }
                }
            }
            _ => {}
        }

        canvas.finish(ctx)
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: event::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        if self.screen == Screen::Menu {
            match button {
                event::MouseButton::Left => {
                    let x_range = self.menu.get_button_range_x();
                    let y_range = self.menu.get_button_range_y();
                    if x_range.0 < x && x_range.1 > x && y_range.0 < y && y_range.1 > y {
                        self.screen = Screen::Game;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        let pressed_key_code = input.keycode.unwrap();
        match pressed_key_code {
            keyboard::KeyCode::W => {
                //conditions rejects if key pressed is same as previous
                //and also rejects moving to reverse direction to current direction
                if self.snake.disallowed_direction() != Direction::UP
                    && self.snake.current_direction != Direction::UP
                    && self.screen == Screen::Game
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
                    && self.screen == Screen::Game
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
                    && self.screen == Screen::Game
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
                    && self.screen == Screen::Game
                {
                    self.snake.last_direction = match self.snake.current_direction.clone() {
                        Direction::NONE => Direction::LEFT,
                        item => item,
                    };
                    self.snake.current_direction = Direction::LEFT
                }
            }
            keyboard::KeyCode::Return => {
                if self.screen == Screen::Menu {
                    self.screen = Screen::Game;
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
