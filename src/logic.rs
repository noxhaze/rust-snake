use crate::renderer;
use array2d::Array2D;

pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub array: Array2D<char>,
}

impl Grid {
    pub fn update(&mut self, snake: &Snake) {
        self.array = Array2D::filled_by_row_major(
            || renderer::BLANK,
            self.width.try_into().unwrap(),
            self.height.try_into().unwrap(),
        );

        for node in snake.nodes.iter() {
            *self
                .array
                .get_mut(node.x.try_into().unwrap(), node.y.try_into().unwrap())
                .expect("Out of bounds access of grid") = renderer::SNAKE;
        }
    }

    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            array: Array2D::filled_by_row_major(
                || renderer::BLANK,
                width.try_into().unwrap(),
                height.try_into().unwrap(),
            ),
        }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

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
                node.y = Snake::wrap_value(node.y, 0, width);
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
