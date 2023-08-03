mod grid;
use std::error::Error;

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

pub fn get_scenic_score(grid: &Grid<usize>, coord: GridCoord) -> Option<i32> {
    let coord_height = grid.get_cell_borrowed(coord).unwrap();
    let dirs: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
    let mut trees_visible_in_dir: Vec<i32> = vec![];

    for dir in dirs.iter() {
        let mut current_coord = coord;
        let mut visible_in_cur_dir = 0;

        loop {
            if current_coord.y.checked_add_signed(dir.1).is_none()
                | current_coord.x.checked_add_signed(dir.0).is_none()
            {
                break;
            }

            current_coord = GridCoord {
                x: current_coord.x.checked_add_signed(dir.0).unwrap(),
                y: current_coord.y.checked_add_signed(dir.1).unwrap(),
            };

            if !grid.in_bounds(current_coord) {
                break;
            }
            visible_in_cur_dir += 1;

            // Break if we find a tree taller than the current coord
            let cur_height = grid.get_cell_borrowed(current_coord).unwrap();
            if cur_height >= coord_height {
                break;
            }
        }

        trees_visible_in_dir.push(visible_in_cur_dir);
    }

    Some(trees_visible_in_dir.iter().product())
}

fn main(grid: &Grid<usize>) -> i32 {
    let all_coords = (0..grid.height()).into_iter().flat_map(|y| {
        (0..grid.width())
            .into_iter()
            .map(move |x| GridCoord { x, y })
    });

    let max_scenic_score = all_coords
        .map(|coord| get_scenic_score(grid, coord))
        .max()
        .unwrap();

    max_scenic_score.unwrap()
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
        let max_scenic_score = main(&grid);
        dbg!(max_scenic_score);
    }

    #[test]
    fn test_scenic_score() {
        let grid_str = "30373
25512
65332
33549
35390";
        let grid = parse_grid(grid_str);
        let scenic_score = get_scenic_score(&grid, GridCoord { x: 2, y: 1 });
        assert!(scenic_score == Some(4));
    }
}
