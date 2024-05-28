use device_query::{DeviceQuery, DeviceState};
use logic::snake::Snake;
use logic::transform;
use std::{thread::sleep, time::Duration};

mod logic;
mod renderer;

fn main() {
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
        renderer::render(&grid);

        let pressed = device_state.get_keys();
        for key in pressed.iter() {
            let new_dir = match key {
                device_query::Keycode::A => transform::Direction::Left,
                device_query::Keycode::S => transform::Direction::Down,
                device_query::Keycode::W => transform::Direction::Up,
                device_query::Keycode::D => transform::Direction::Right,
                _ => continue,
            };

            if new_dir != snake.dir.opposite() {
                snake.dir = new_dir;
            }
        }

        if *snake.nodes.get(0).unwrap() == food.pos {
            food.pos = grid.random_pos();
            snake.increase_length();
        }

        snake.update(width, height);
        grid.update(&snake, &food);

        sleep(Duration::from_millis(DELAY));
    }
}
