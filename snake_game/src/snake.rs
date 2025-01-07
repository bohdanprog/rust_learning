#[derive(Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}

pub struct Snake {
    pub body: std::collections::LinkedList<Block>,
    pub dir: Direction,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        let mut body: std::collections::LinkedList<Block> = std::collections::LinkedList::new();
        body.push_back(Block { x, y });
        body.push_back(Block { x: x - 1, y });

        Self {
            body,
            dir: Direction::Right,
        }
    }

    pub fn move_forward(&mut self, grow: bool) {
        let mut new_head = (*self.body.front().unwrap()).clone();

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
        if head.x < 0 || head.x >= 20 || head.y < 0 || head.y >= 20 {
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

