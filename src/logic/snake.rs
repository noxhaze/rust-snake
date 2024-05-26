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
                match self.dir {
                    Direction::Left => node.x -= 1,
                    Direction::Right => node.x += 1,
                    Direction::Up => node.y -= 1,
                    Direction::Down => node.y += 1,
                }

                node.x = Snake::wrap_value(node.x, 0, width);
                node.y = Snake::wrap_value(node.y, 0, height);
            } else {
                *node = old_pos;
            }

            old_pos = node.clone();
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
