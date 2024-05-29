use device_query::{DeviceQuery, DeviceState};
use logic::snake::Snake;
use logic::transform;
use std::{thread::sleep, time::Duration};

mod logic;
mod renderer;

fn main() -> ! {
    const DELAY: u64 = 100;
    let width: u32 = 25;
    let height: u32 = 10;
    let device_state: DeviceState = DeviceState::new();

    let mut grid: logic::Grid = logic::Grid::new(width, height);
    let mut snake: Snake = Snake::new(
        transform::Position {
            x: width / 2,
            y: height / 2,
        },
        transform::Direction::Down,
    );
    let mut food: logic::Food = logic::Food {
        pos: transform::Position { x: 1, y: 5 },
    };

    loop {
        if let Some(val) = device_state.get_keys().get(0) {
            snake.handle_input(*val);
        }

        if *snake.nodes.get(0).unwrap() == food.pos {
            food.pos = grid.random_pos();
            snake.increase_length();
        }

        snake.update(width, height);
        grid.update(&snake, &food);
        renderer::render(&grid);

        sleep(Duration::from_millis(DELAY));
    }
}
