mod logic;
mod renderer;

fn main() {
    let width: u32 = 10;
    let height: u32 = 10;

    let mut grid: logic::Grid = logic::Grid::new(width, height);
    let mut snake: logic::Snake = logic::Snake::new(
        logic::Position {
            x: width / 2,
            y: height / 2,
        },
        logic::Direction::Down,
    );

    loop {
        renderer::render(&grid);

        grid.update(&snake);
        snake.update();
    }
}
