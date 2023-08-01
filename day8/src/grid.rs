use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GridCoord {
    pub x: usize,
    pub y: usize,
}
impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    data: Vec<T>,
}

impl Debug for Grid<usize> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut row_strings: Vec<String> = vec![String::new()];

        for row in 0..self.height {
            let start_index = row * self.width;
            let end_index = (row + 1) * self.width;
            let row_vec: Vec<String> = self.data[start_index..end_index]
                .iter()
                .map(|n| u32::try_from(n.clone()).unwrap().to_string())
                .collect();

            row_strings.push(row_vec.join(" "));
        }

        write!(f, "{}", row_strings.join("\n"))
    }
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![T::default(); width * height],
        }
    }

    pub fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    pub fn get_cell_mut(&mut self, coord: GridCoord) -> Option<&mut T> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&mut self.data[coord.y * self.width + coord.x])
    }

    pub fn get_cell_borrowed(&self, cord: GridCoord) -> Option<&T> {
        if !self.in_bounds(cord) {
            return None;
        }
        Some(&self.data[cord.y * self.width + cord.x])
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
