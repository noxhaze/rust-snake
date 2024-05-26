use logic::snake::Snake;
use logic::transform;
use std::{thread::sleep, time::Duration};

mod logic;
mod renderer;

fn main() {
    let width: u32 = 10;
    let height: u32 = 10;

    let mut grid: logic::Grid = logic::Grid::new(width, height);
    let mut snake: Snake = Snake::new(
        transform::Position {
            x: width / 2,
            y: height / 2,
        },
        transform::Direction::Down,
    );

    loop {
        renderer::render(&grid);

        grid.update(&snake);
        snake.update(width, height);

        sleep(Duration::from_millis(250));
    }
}
