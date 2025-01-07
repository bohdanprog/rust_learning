use crate::snake::{Block, Snake};
use rand::Rng;

pub struct Game {
    pub snake: Snake,
    pub food: Block,
    pub grow: bool,
    pub game_over: bool,
    pub welcome: bool,
}

impl Game {
    pub fn new() -> Self {
        Self {
            snake: Snake::new(10, 10),
            food: Game::new_food(),
            grow: false,
            game_over: false,
            welcome: true
        }
    }

    fn new_food() -> Block {
        let mut rng = rand::thread_rng();
        Block {
            x: rng.gen_range(0..20),
            y: rng.gen_range(0..20),
        }
    }

    pub fn update(&mut self) {
        if self.game_over || self.welcome {
            return;
        }

        let head = self.snake.head_position();
        if head.x == self.food.x && head.y == self.food.y {
            self.food = Game::new_food();
            self.grow = true;
        }
        self.snake.move_forward(self.grow);
        self.grow = false;

        if self.snake.check_collision() {
            self.game_over = true;
        }
    }

    pub fn reset(&mut self) {
        *self = Game::new();
    }
}