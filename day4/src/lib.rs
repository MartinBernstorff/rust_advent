use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct ZoneAssignment {
    min: u32,
    max: u32,
}
impl ZoneAssignment {
    fn has_overlap(&self, other: Self) -> bool {
        !(self.min > other.max || self.max < other.min)
    }
}

impl TryFrom<&str> for ZoneAssignment {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // Parse string into values of u32
        let values = value
            .split('-')
            .map(|x| x.parse::<u32>())
            .map(|x| x.unwrap())
            .collect::<Vec<u32>>();

        let (min, max) = match (values.iter().min(), values.iter().max()) {
            (Some(min), Some(max)) => (*min, *max),
            _ => return Err(eyre!("Invalid input")),
        };

        Ok(ZoneAssignment { min, max })
    }
}

pub fn parse_elf_pairs(input: &str) -> Vec<ZoneAssignment> {
    input
        .split(',')
        .map(|x| ZoneAssignment::try_from(x).unwrap())
        .collect::<Vec<ZoneAssignment>>()
}

mod tests {
    use super::*;

    #[test]
    fn run_main() {
        let input = include_str!("input.txt");
        let elf_pairs = input.lines().map(parse_elf_pairs).collect::<Vec<_>>();

        let mut has_overlap = 0;
        for pair in elf_pairs.iter() {
            if pair[0].has_overlap(pair[1]) || pair[1].has_overlap(pair[0]) {
                has_overlap += 1;
            }
        }

        dbg!(has_overlap);
    }

    #[test]
    fn test_parse_elf_pair() {
        let input = "1-3,5-7";
        let result = parse_elf_pairs(input);

        assert_eq!(
            result,
            vec![
                ZoneAssignment { min: 1, max: 3 },
                ZoneAssignment { min: 5, max: 7 }
            ]
        );
    }

    #[test]
    fn test_no_overlap() {
        let assignment = ZoneAssignment { min: 1, max: 3 };
        let other = ZoneAssignment { min: 0, max: 4 };
        assert!(assignment.has_overlap(other));

        let other_no_overlap = ZoneAssignment { min: 4, max: 5 };
        assert!(!assignment.has_overlap(other_no_overlap));
    }

    #[test]
    fn test_assignment_parsing() {
        let assignment = ZoneAssignment::try_from("1-3").unwrap();
        assert_eq!(assignment.min, 1);
        assert_eq!(assignment.max, 3);
    }
}
