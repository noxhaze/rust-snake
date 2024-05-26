use crate::renderer;
use array2d::Array2D;

pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub array: Array2D<char>,
}

impl Grid {
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
