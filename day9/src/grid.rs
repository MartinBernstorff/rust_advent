#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl std::ops::Add for GridPos {
    type Output = GridPos;

    fn add(self, other: GridPos) -> GridPos {
        GridPos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for GridPos {
    type Output = GridPos;

    fn sub(self, other: GridPos) -> GridPos {
        GridPos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
