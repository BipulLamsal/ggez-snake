use core::f32;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color, DrawMode, FillOptions};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, GameResult};
use rand::Rng;

const DIMENSION_SIZE: (i32, i32) = (20, 20);
const GRID_SIZE: (i32, i32) = (32, 32);
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * DIMENSION_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * DIMENSION_SIZE.1 as f32,
);
const FPS: u32 = 10;

struct BackgroundGrid;
// enum GridPosition {
//     X(i32),
//     Y(i32),
// }

struct GridPosition {
    x: i32,
    y: i32,
}
impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        GridPosition { x, y }
    }
}
impl From<(i32, i32)> for GridPosition {
    fn from(pos: (i32, i32)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}
impl BackgroundGrid {
    fn draw(ctx: &mut Context, canvas: &mut Canvas, x: f32, y: f32) {
        let math_value = ((x + y) as i32 / GRID_SIZE.0 as i32) % 2;
        let color = if math_value == 0 {
            Color::from_rgb(196, 229, 56)
        } else {
            Color::from_rgb(163, 203, 56)
        };

        let mut binding = graphics::MeshBuilder::new();
        let _grid_builder = binding.rectangle(
            DrawMode::Fill(FillOptions::default()),
            graphics::Rect {
                x,
                y,
                w: GRID_SIZE.0 as f32,
                h: GRID_SIZE.1 as f32,
            },
            color,
        );
        let grid = graphics::Mesh::from_data(ctx, binding.build());
        canvas.draw(&grid, graphics::DrawParam::default());
    }
}
struct MainGame {
    food: Food,
}
impl MainGame {
    fn init() -> GameResult<MainGame> {
        let food = Food::generate();
        Ok(MainGame { food })
    }
}
impl event::EventHandler<ggez::GameError> for MainGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        for i in (0..SCREEN_SIZE.0 as i32 + 1).step_by(32) {
            for j in (0..SCREEN_SIZE.1 as i32 + 1).step_by(32) {
                BackgroundGrid::draw(ctx, &mut canvas, i as f32, j as f32);
            }
        }
        self.food.draw(ctx, &mut canvas);
        // canvas.draw(&self.background.square, graphics::DrawParam::default());
        canvas.finish(ctx)
    }
}

// trati for fucking around
trait GridGeneration {
    fn generate() -> Self;
    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas);
}
struct Food {
    pos: GridPosition,
}

impl GridGeneration for Food {
    fn generate() -> Food {
        let mut rng = rand::thread_rng();
        Food {
            pos: (
                rng.gen_range(0..(GRID_SIZE.0 + 1)) * GRID_SIZE.0 as i32,
                rng.gen_range(0..(GRID_SIZE.0 + 1)) * GRID_SIZE.0 as i32,
            )
                .into(),
        }
    }
    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) {
        let mut binding = graphics::MeshBuilder::new();
        println!("{} {}", self.pos.x, self.pos.y);
        let _grid_builder = binding.rectangle(
            DrawMode::Fill(FillOptions::default()),
            graphics::Rect {
                x: self.pos.x as f32,
                y: self.pos.y as f32,
                w: GRID_SIZE.0 as f32,
                h: GRID_SIZE.1 as f32,
            },
            Color::from_rgb(238, 90, 36),
        );
        let grid = graphics::Mesh::from_data(ctx, binding.build());
        canvas.draw(&grid, graphics::DrawParam::default());
    }
}
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn inverse(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
    pub fn from_keycode(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Up => Some(Direction::Up),
            KeyCode::Down => Some(Direction::Down),
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

struct Snake {
    head: Segment,
    direction: Direction,
}

struct Segment {
    pos: GridPosition,
}
impl Segment {
    pub fn new(pos: GridPosition) -> Self {
        Segment { pos }
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
