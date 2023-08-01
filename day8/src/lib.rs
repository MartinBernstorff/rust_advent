mod grid;
use grid::{Grid, GridCoord};

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

pub fn main(grid: Grid<usize>) {
    let all_coords = (0..grid.height()).into_iter().flat_map(|y| {
        (0..grid.width())
            .into_iter()
            .map(move |x| GridCoord::from((x, y)))
    });

    let num_visible_cells = all_coords
        .filter(|&coord| {
            let cur_coord_height = grid.get_cell_borrowed(coord).unwrap();
            let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

            directions.iter().any(|&(dx, dy)| {
                // Find all indeces from where all cells are lower in any direction
                // These indices are visible from outside the grid
                let mut cells_in_direction = (1..).into_iter().map_while(|i| {
                    let coord = GridCoord {
                        x: coord.x.checked_add_signed(dx * i)?,
                        y: coord.y.checked_add_signed(dy * i)?,
                    };
                    grid.get_cell_borrowed(coord) // Returns None if out of bounds.
                });
                cells_in_direction.all(|height| height < cur_coord_height)
            })
        })
        .count();

    dbg!(num_visible_cells);
}

#[cfg(test)]
mod tests {
    use crate::grid::GridCoord;

    use super::*;

    #[test]
    fn test_parse_grid() {
        let input = include_str!("initial_input.txt");
        let grid = parse_grid(input);
        assert!(grid.get_cell_borrowed((0, 0).into()).unwrap() == &3);
        dbg!(grid);
    }

    #[test]
    fn test_main() {
        let input = include_str!("full_input.txt");
        let grid = parse_grid(input);
        let result = main(grid);
        dbg!(result);
    }
}
