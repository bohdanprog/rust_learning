use std::collections::LinkedList;
use crate::{Block, Direction, HEIGHT, WIDTH};

pub struct Snake {
    pub body: LinkedList<Block>,
    pub dir: Direction,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x, y });
        body.push_back(Block { x: x - 1, y });
        Snake {
            body,
            dir: Direction::Right,
        }
    }

    pub fn move_forward(&mut self, grow: bool) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        match self.dir {
            Direction::Up => new_head.y -= 1,
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x -= 1,
            Direction::Right => new_head.x += 1,
        }

        self.body.push_front(new_head);
        if !grow {
            self.body.pop_back();
        }
    }

    pub fn change_direction(&mut self, dir: Direction) {
        if (self.dir == Direction::Up && dir != Direction::Down)
            || (self.dir == Direction::Down && dir != Direction::Up)
            || (self.dir == Direction::Left && dir != Direction::Right)
            || (self.dir == Direction::Right && dir != Direction::Left)
        {
            self.dir = dir;
        }
    }

    pub fn check_collision(&self) -> bool {
        let head = self.body.front().unwrap();
        if head.x < 0 || head.x >= WIDTH || head.y < 0 || head.y >= HEIGHT {
            return true;
        }

        for block in self.body.iter().skip(1) {
            if block.x == head.x && block.y == head.y {
                return true;
            }
        }

        false
    }

    pub fn head_position(&self) -> Block {
        *self.body.front().unwrap()
    }
}
