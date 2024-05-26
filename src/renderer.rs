pub const BLANK: char = '.';
pub const SNAKE: char = '#';
pub const FOOD: char = '@';

use crate::logic;

pub fn render(grid: &logic::Grid) {
    // clears the screen
    print!("\x1B[2J\x1B[1;1H");

    let mut output: String = String::new();

    for iter in grid.array.rows_iter() {
        for element in iter {
            output.push(*element);
        }
        output.push('\n');
    }

    println!("{}", output);
}
