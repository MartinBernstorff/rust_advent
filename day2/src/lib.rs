/*  Other elf,
    A: Rock,
    B: Paper,
    C: Scissors

    You:
    X: Rock
    Y: Paper
    Z: Scissors

    Points:
        Shape you selected (1: X, 2: Y, 3: Z)
        Outcome of round:
            Loss: 0
            Draw: 3
            Win: 6

    Get sum of points if everything goes according to the strategy guide.
*/

use color_eyre::owo_colors::OwoColorize;

pub fn load_input() -> &'static str {
    include_str!("day2.txt")
}

#[derive(Debug, Clone, Copy)]
pub struct Round {
    ours: Move,
    theirs: Move,
}

impl Round {
    pub fn points(&self) -> u32 {
        let victory_points = if self.ours == self.theirs {
            3
        } else if self.ours.wins_against(&self.theirs) {
            6
        } else {
            0
        };

        let our_move_points = if self.ours == Move::Rock {
            1
        } else if self.ours == Move::Paper {
            2
        } else {
            3
        };

        victory_points + our_move_points
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn wins_against(&self, other: &Move) -> bool {
        match (self, other) {
            (Move::Rock, Move::Scissors) => true,
            (Move::Paper, Move::Rock) => true,
            (Move::Scissors, Move::Paper) => true,
            _ => false,
        }
    }
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("Invalid move")),
        }
    }
}

pub fn parse_input_to_moves(input: &str) -> Vec<Round> {
    // Load input
    let char_pairs = str_to_char_pairs(input);
    char_pairs_to_moves(char_pairs)
}

fn char_pairs_to_moves(char_pairs: Vec<(&str, &str)>) -> Vec<Round> {
    char_pairs
        .iter()
        .map(|(first, second)| {
            let their_char = first.chars().next().unwrap();
            let our_char = second.chars().next().unwrap();

            Round {
                theirs: Move::try_from(their_char).unwrap(),
                ours: Move::try_from(our_char).unwrap(),
            }
        })
        .collect::<Vec<_>>()
}

fn str_to_char_pairs(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| {
            if let Some((first, second)) = line.split_once(' ') {
                (first, second)
            } else {
                panic!("Invalid input")
            }
        })
        .collect::<Vec<_>>()
}

pub fn lib_main(input: &str) {
    let moves = parse_input_to_moves(input);
    let total_points: u32 = moves.iter().map(|round| round.points()).sum();

    println!("{:?}", total_points)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        lib_main("A Y\nB X\nC Z")
    }

    #[test]
    fn test_parse_str_to_move_pairs() {
        let move_pairs = str_to_char_pairs("A X\nB Y\nC Z");
        assert_eq!(move_pairs, vec![("A", "X"), ("B", "Y"), ("C", "Z")]);
    }

    #[test]
    fn test_str_to_move_enum() {
        let current_move = Move::try_from('A').unwrap();
        assert_eq!(current_move, Move::Rock);
    }
}
