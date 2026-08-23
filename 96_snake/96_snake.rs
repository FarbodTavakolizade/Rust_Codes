// main.rs

/* Cargo.toml should be like this
[package]
name = "snake"
version = "0.1.0"
edition = "2024"

[dependencies]
piston_window="0.127.0"
rand="0.8" 
*/

use piston_window::*;
use rand::Rng;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

const BLOCK_SIZE: f64 = 25.0;
const WIDTH: u32 = 20;
const HEIGHT: u32 = 20;
const FPS: u64 = 10;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Block {
    x: u32,
    y: u32,
}

struct Snake {
    body: VecDeque<Block>,
    dir: Direction,
    pending_dir: Direction,
}

impl Snake {
    fn new() -> Self {
        let mut body = VecDeque::new();
        body.push_back(Block { x: 10, y: 10 });
        Snake {
            body,
            dir: Direction::Right,
            pending_dir: Direction::Right,
        }
    }

    fn head(&self) -> Block {
        *self.body.front().unwrap()
    }

    fn move_forward(&mut self, grow: bool) {
        let mut new_head = self.head();
        match self.dir {
            Direction::Up => {
                if new_head.y > 0 {
                    new_head.y -= 1;
                } else {
                    new_head.y = HEIGHT - 1;
                }
            }
            Direction::Down => {
                new_head.y = (new_head.y + 1) % HEIGHT;
            }
            Direction::Left => {
                if new_head.x > 0 {
                    new_head.x -= 1;
                } else {
                    new_head.x = WIDTH - 1;
                }
            }
            Direction::Right => {
                new_head.x = (new_head.x + 1) % WIDTH;
            }
        }
        self.body.push_front(new_head);
        if !grow {
            self.body.pop_back();
        }
    }

    fn overlaps_tail(&self, pos: &Block) -> bool {
        self.body.iter().skip(1).any(|b| b == pos)
    }

    fn set_direction(&mut self, dir: Direction) {
        if dir != self.dir.opposite() {
            self.pending_dir = dir;
        }
    }

    fn update_direction(&mut self) {
        self.dir = self.pending_dir;
    }
}

struct Game {
    snake: Snake,
    food: Block,
    width: u32,
    height: u32,
    game_over: bool,
    last_update: Instant,
}

impl Game {
    fn new(width: u32, height: u32) -> Self {
        let mut game = Game {
            snake: Snake::new(),
            food: Block { x: 0, y: 0 },
            width,
            height,
            game_over: false,
            last_update: Instant::now(),
        };
        game.add_food();
        game
    }

    fn add_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            let new_food = Block { x, y };
            if !self.snake.body.contains(&new_food) {
                self.food = new_food;
                break;
            }
        }
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }

        if self.last_update.elapsed() < Duration::from_millis(1000 / FPS) {
            return;
        }

        self.snake.update_direction();
        let next_head = {
            let mut new_head = self.snake.head();
            match self.snake.dir {
                Direction::Up => {
                    if new_head.y > 0 {
                        new_head.y -= 1;
                    } else {
                        new_head.y = self.height - 1;
                    }
                }
                Direction::Down => {
                    new_head.y = (new_head.y + 1) % self.height;
                }
                Direction::Left => {
                    if new_head.x > 0 {
                        new_head.x -= 1;
                    } else {
                        new_head.x = self.width - 1;
                    }
                }
                Direction::Right => {
                    new_head.x = (new_head.x + 1) % self.width;
                }
            }
            new_head
        };

        if self.snake.overlaps_tail(&next_head) {
            self.game_over = true;
            return;
        }

        let ate = next_head == self.food;
        self.snake.move_forward(ate);
        if ate {
            self.add_food();
        }

        self.last_update = Instant::now();
    }

    fn key_pressed(&mut self, key: Key) {
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };
        if let Some(d) = dir {
            self.snake.set_direction(d);
        }
    }

    fn draw(&self, ctx: &Context, g: &mut G2d) {
        clear([0.0, 0.0, 0.0, 1.0], g);

        // Draw snake
        for block in &self.snake.body {
            rectangle(
                [0.0, 1.0, 0.0, 1.0],
                [
                    (block.x as f64) * BLOCK_SIZE,
                    (block.y as f64) * BLOCK_SIZE,
                    BLOCK_SIZE,
                    BLOCK_SIZE,
                ],
                ctx.transform,
                g,
            );
        }

        // Draw food
        rectangle(
            [1.0, 0.5, 0.0, 1.0],
            [
                (self.food.x as f64) * BLOCK_SIZE,
                (self.food.y as f64) * BLOCK_SIZE,
                BLOCK_SIZE,
                BLOCK_SIZE,
            ],
            ctx.transform,
            g,
        );

        if self.game_over {
            // Optionally, draw game over text here
        }
    }
}

fn main() {
    let (width, height) = (WIDTH, HEIGHT);
    let window_size = [BLOCK_SIZE * width as f64, BLOCK_SIZE * height as f64];

    let mut window: PistonWindow = WindowSettings::new("Snake Game", window_size)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        if let Some(_) = event.update_args() {
            game.update();
        }

        window.draw_2d(&event, |ctx, g, _| {
            game.draw(&ctx, g);
        });
    }
}
