use color_eyre::eyre::{eyre, Result};
use itertools::Itertools;
#[derive(PartialEq, Debug, Clone)]
pub struct ZoneAssignment {
    range: std::ops::RangeInclusive<u32>,
}
impl ZoneAssignment {
    fn has_overlap(&self, other: Self) -> bool {
        !(self.range.start() > other.range.end() || self.range.end() < other.range.start())
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

        Ok(ZoneAssignment { range: min..=max })
    }
}

pub fn parse_elf_pairs(input: &str) -> Vec<ZoneAssignment> {
    input
        .split(',')
        .map(|x| ZoneAssignment::try_from(x).unwrap())
        .collect::<Vec<ZoneAssignment>>()
}

fn main() {
    println!("Testing!");
}

mod tests {
    use super::*;

    #[test]
    fn run_main() {
        let input = include_str!("input.txt");
        let elf_pairs = input.lines().map(parse_elf_pairs).collect::<Vec<_>>();

        let mut has_overlap = 0;
        for pair in elf_pairs.iter() {
            let first = &pair[0];
            let second = &pair[1];
            if first.has_overlap(second.clone()) {
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
                ZoneAssignment { range: 1..=3 },
                ZoneAssignment { range: 5..=7 }
            ]
        );
    }

    #[test]
    fn test_no_overlap() {
        let assignment = ZoneAssignment { range: 1..=3 };
        let other = ZoneAssignment { range: 0..=4 };
        assert!(assignment.has_overlap(other));

        let other_no_overlap = ZoneAssignment { range: 4..=5 };
        assert!(!assignment.has_overlap(other_no_overlap));
    }

    #[test]
    fn test_assignment_parsing() {
        let assignment = ZoneAssignment::try_from("1-3").unwrap();
        assert_eq!(assignment.range.start(), &1);
        assert_eq!(assignment.range.end(), &3);
    }
}
