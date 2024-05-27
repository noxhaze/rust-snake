use core::panic;

use crate::logic::transform::{Direction, Position};

pub struct Snake {
    pub nodes: Vec<Position>,
    pub dir: Direction,
}

impl Snake {
    pub fn update(&mut self, width: u32, height: u32) {
        let mut old_pos: Position = Position { x: 0, y: 0 };
        let first_node = self.nodes.first().unwrap().clone();
        for node in self.nodes.iter_mut() {
            if *node == first_node {
                old_pos = node.clone();

                match self.dir {
                    Direction::Left => node.x -= 1,
                    Direction::Right => node.x += 1,
                    Direction::Up => node.y -= 1,
                    Direction::Down => node.y += 1,
                };

                node.x = Snake::wrap_value(node.x, 0, width);
                node.y = Snake::wrap_value(node.y, 0, height);
            } else {
                *node = old_pos;
                old_pos = node.clone();
            }
        }
    }

    pub fn check_self_collisions(&self) -> bool {
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
        let x: i32 = TryInto::<i32>::try_into(first.x).unwrap()
            - TryInto::<i32>::try_into(second.x).unwrap();
        let y: i32 = TryInto::<i32>::try_into(first.y).unwrap()
            - TryInto::<i32>::try_into(second.y).unwrap();

        if x != 0 {
            match x {
                -1 => Direction::Left,
                1 => Direction::Right,
                _ => panic!("Unaccounted direction calcuation"),
            }
        } else {
            match y {
                -1 => Direction::Up,
                1 => Direction::Down,
                _ => panic!("Unaccounted direction calcuation"),
            }
        }
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
