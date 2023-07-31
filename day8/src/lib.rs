mod grid;
use grid::Grid;

pub fn parse_grid(input: &str) -> Grid<usize> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut grid = Grid::new(width, height);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            assert!(c.is_ascii_digit());
            *grid.get_cell_mut((x, y).into()).unwrap() = c as usize - '0' as usize;
        }
    }

    grid
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid() {
        let input = include_str!("initial_input.txt");
        let grid = parse_grid(input);
        dbg!(grid);
    }
}
