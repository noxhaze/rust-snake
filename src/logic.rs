use crate::renderer;
use array2d::Array2D;

pub mod snake;
pub mod transform;

pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub array: Array2D<char>,
}

impl Grid {
    pub fn update(&mut self, snake: &snake::Snake) {
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
