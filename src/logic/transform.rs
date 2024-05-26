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
