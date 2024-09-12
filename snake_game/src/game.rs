use rand::Rng;
use crate::{Block, HEIGHT, WIDTH};
use crate::snake::Snake;

pub struct Game {
    pub snake: Snake,
    pub food: Block,
    pub grow: bool,
    pub game_over: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            snake: Snake::new(WIDTH / 2, HEIGHT / 2),
            food: Game::new_food(),
            grow: false,
            game_over: false,
        }
    }

    pub fn new_food() -> Block {
        let mut rng = rand::thread_rng();
        Block {
            x: rng.gen_range(0..WIDTH),
            y: rng.gen_range(0..HEIGHT),
        }
    }

    pub fn update(&mut self) {
        if self.game_over {
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