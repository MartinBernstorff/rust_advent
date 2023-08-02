use grid::GridPos;
use instruction_parsing::Instruction;
use itertools::Itertools;

pub mod grid;
pub mod instruction_parsing;

fn main(instructions: Vec<Instruction>) -> usize {
    let mut knots = [GridPos { x: 0, y: 0 }; 10];
    let mut tail_positions_visited = vec![knots[0].clone()];

    for i in instructions {
        let mut head_position = knots[0];

        match i {
            Instruction::Up => {
                head_position.y += 1;
            }
            Instruction::Down => {
                head_position.y -= 1;
            }
            Instruction::Left => {
                head_position.x -= 1;
            }
            Instruction::Right => {
                head_position.x += 1;
            }
        }
        knots[0] = head_position;

        // Process each knot after the head
        for i in 1..knots.len() {
            // 1 to skip the head
            let diff = knots[i - 1] - knots[i];
            let (dx, dy) = update_position(diff);

            knots[i] = GridPos {
                x: knots[i].x + dx,
                y: knots[i].y + dy,
            };

            if i == knots.len() - 1 {
                tail_positions_visited.push(knots[i]);
            }
        }
    }

    // Get unique positions visited by the tail with a HashSet
    let unique_tail_positions: Vec<&GridPos> = tail_positions_visited.iter().unique().collect();

    println!(
        "The tail visited {} unique positions",
        unique_tail_positions.len()
    );
    unique_tail_positions.len()
}

fn update_position(position_delta: GridPos) -> (i32, i32) {
    let (dx, dy) = match (position_delta.x, position_delta.y) {
        // overlapping
        (0, 0) => (0, 0),
        // touching up/left/down/right
        (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
        // touching diagonally
        (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
        // need to move up/left/down/right
        (0, 2) => (0, 1),
        (0, -2) => (0, -1),
        (2, 0) => (1, 0),
        (-2, 0) => (-1, 0),
        // need to move to the right diagonally
        (2, 1) => (1, 1),
        (2, -1) => (1, -1),
        // need to move to the left diagonally
        (-2, 1) => (-1, 1),
        (-2, -1) => (-1, -1),
        // As before, move diagonally
        (1, 2) => (1, 1),
        (-1, 2) => (-1, 1),
        (1, -2) => (1, -1),
        (-1, -2) => (-1, -1),
        // Big diagonal moves, since the prior knot can move diagonally
        (-2, -2) => (-1, -1),
        (-2, 2) => (-1, 1),
        (2, -2) => (1, -1),
        (2, 2) => (1, 1),
        _ => panic!("{:?}", position_delta),
    };
    (dx, dy)
}
#[cfg(test)]
mod tests {
    use crate::instruction_parsing::{parse_line, Instruction};

    use super::*;
    #[test]
    fn test_full_input() {
        let input = include_str!("full_input.txt");
        let lines = input.lines();
        let instructions = lines.map(|l| parse_line(l)).flatten().collect();

        let positions_visited = main(instructions);
    }
    #[test]
    fn test_starting_input() {
        let input = include_str!("sample_input.txt");
        let lines = input.lines();
        let instructions = lines.map(|l| parse_line(l)).flatten().collect();

        let positions_visited = main(instructions);
        assert_eq!(positions_visited, 13);
    }

    #[test]
    fn test_parse_line() {
        let line = "U 10";
        let instructions = parse_line(line);
        assert_eq!(instructions.len(), 10);
        assert_eq!(instructions[0], Instruction::Up);
        assert_eq!(instructions[1], Instruction::Up);
    }
}
