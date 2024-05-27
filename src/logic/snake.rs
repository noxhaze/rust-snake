use crate::logic::transform::{Direction, Position};

pub struct Snake {
    pub nodes: Vec<Position>,
    pub dir: Direction,
}

impl Snake {
    pub fn update(&mut self, width: u32, height: u32) {
        let head = self.nodes.get_mut(0).unwrap();
        let mut old_pos = head.clone();
        match self.dir {
            Direction::Left => head.x -= 1,
            Direction::Right => head.x += 1,
            Direction::Up => head.y -= 1,
            Direction::Down => head.y += 1,
        };
        head.x = Snake::wrap_value(head.x, 0, width);
        head.y = Snake::wrap_value(head.y, 0, height);

        for node in self.nodes[1..].iter_mut() {
            *node = old_pos;
            old_pos = node.clone();
        }

        self.check_self_collisions();
    }

    fn check_self_collisions(&self) -> bool {
        let mut res: bool = false;
        let head_pos: &Position = self.nodes.get(0).unwrap();

        for node in self.nodes[1..].iter() {
            if node == head_pos {
                res = true;
            }
        }

        res
    }

    pub fn increase_length(&mut self) {
        let last = self.nodes.last().unwrap();

        if self.nodes.len() == 1 {
            self.nodes.push(Snake::follow(&last, &self.dir));
        } else {
            let second_last = self.nodes.get(self.nodes.len() - 2).unwrap();
            let dir = Snake::dir(second_last, last);
            self.nodes.push(Snake::follow(&last, &dir));
        }
    }

    fn dir(first: &Position, second: &Position) -> Direction {
        todo!("Calculating direction function was previously broken entirely")
    }

    fn follow(pos: &Position, dir: &Direction) -> Position {
        match dir {
            Direction::Left => Position {
                x: pos.x + 1,
                y: pos.y,
            },
            Direction::Right => Position {
                x: pos.x - 1,
                y: pos.y,
            },
            Direction::Down => Position {
                x: pos.x,
                y: pos.y - 1,
            },
            Direction::Up => Position {
                x: pos.x,
                y: pos.y + 1,
            },
        }
    }

    fn wrap_value(val: u32, min: u32, max: u32) -> u32 {
        if val <= min {
            max - 1
        } else if val >= max {
            min
        } else {
            val
        }
    }

    pub fn new(start_pos: Position, start_dir: Direction) -> Self {
        Self {
            nodes: vec![start_pos],
            dir: start_dir,
        }
    }
}
