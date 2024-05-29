use core::panic;

use crate::logic::transform::{Direction, Position};

pub struct Snake {
    pub nodes: Vec<Position>,
    pub dir: Direction,
}

impl Snake {
    pub fn update(&mut self, width: u32, height: u32) {
        let old: Vec<Position> = self.nodes.clone();
        let head = self.nodes.get_mut(0).unwrap();

        match self.dir {
            Direction::Left => head.x -= 1,
            Direction::Right => head.x += 1,
            Direction::Up => head.y -= 1,
            Direction::Down => head.y += 1,
        };
        head.x = Snake::wrap_value(head.x, 0, width);
        head.y = Snake::wrap_value(head.y, 0, height);

        let mut i: usize = 0;
        for node in self.nodes.iter_mut().skip(1) {
            *node = old.get(i).unwrap().clone();
            i += 1;
        }

        if self.check_self_collisions() {
            panic!("You Lost!");
        }
    }

    fn check_self_collisions(&self) -> bool {
        let mut res: bool = false;
        let head_pos: &Position = self.nodes.get(0).unwrap();

        for node in self.nodes.iter().skip(1) {
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
            self.nodes.push(Snake::follow(&last, &self.dir));
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
        let second_node: Position = Position {
            x: start_pos.x,
            y: start_pos.y - 1,
        };

        Self {
            nodes: vec![start_pos, second_node],
            dir: start_dir,
        }
    }
}
