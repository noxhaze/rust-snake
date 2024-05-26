pub const BLANK: char = '.';
pub const SNAKE: char = '#';
pub const FOOD: char = '@';

use crate::logic;

pub fn render(grid: &logic::Grid) {
    for x in 0..=grid.width {
        for y in 0..=grid.height {}
    }
}
