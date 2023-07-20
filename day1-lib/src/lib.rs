use itertools::Itertools;
use std::cmp::Reverse;

pub fn load_input_file() -> &'static str {
    let input = include_str!("day1.txt");
    input
}
pub fn parse_string_to_elves(input: &str) -> Vec<u32> {
    // Newlines on Windows are \r\n
    let lines = input.lines();

    // Remove the None values
    lines
        .map(|v| v.parse::<u32>().ok())
        .batching(|l| {
            let mut sum = None;
            while let Some(Some(v)) = l.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .collect()
}

pub fn get_sum_of_top_n(input: Vec<i32>, top_n: usize) -> i32 {
    input
        .into_iter()
        .map(Reverse)
        .k_smallest(top_n)
        .map(|v| v.0)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::load_input_file;

    #[test]
    fn test_load_file() {
        load_input_file();
    }

    #[test]
    fn test_parse_string_to_elves() {
        let input = "1\n2\n\n4\n\n";
        let actual = super::parse_string_to_elves(input);
        let expected = vec![3, 4];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_sum_of_top_n() {
        let input = vec![1, 2, 3];
        let actual = super::get_sum_of_top_n(input, 2);
        let expected = 5;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_gets_sum_of_n() {}
}
