use crate::renderer;
use array2d::Array2D;
use rand::Rng;

use self::transform::Position;

pub mod snake;
pub mod transform;

pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub array: Array2D<char>,
    rng: rand::rngs::ThreadRng,
}

impl Grid {
    pub fn update(&mut self, snake: &snake::Snake, food: &Food) {
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

        *self
            .array
            .get_mut(
                food.pos.x.try_into().unwrap(),
                food.pos.y.try_into().unwrap(),
            )
            .expect("Out of bounds access of grid") = renderer::FOOD;
    }

    pub fn random_pos(&mut self) -> Position {
        Position {
            x: self.rng.gen_range(0..self.width),
            y: self.rng.gen_range(0..self.height),
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
            rng: rand::thread_rng(),
        }
    }
}

pub struct Food {
    pub pos: transform::Position,
}
