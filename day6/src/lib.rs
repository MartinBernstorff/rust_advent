use itertools::Itertools;
use std::collections::HashSet;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct TestInput<'a> {
    input: &'a str,
    expected_index: usize,
}

fn get_position_of_input(input: &TestInput, window_size: u32) -> Option<usize> {
    get_end_index_of_first_unique_chunk(input.input, window_size).map(|pos| pos)
}

fn get_end_index_of_first_unique_chunk(input: &str, window_size: u32) -> Option<usize> {
    let window_usize = window_size as usize;
    input
        .as_bytes()
        .windows(window_usize)
        .position(|window| window.iter().unique().count() == window_usize)
        .map(|pos| pos + window_usize)
}

fn get_char_at_index(input: &str, index: usize) -> Option<char> {
    input.chars().nth(index)
}

#[cfg(test)]
mod tests {
    use std::usize;

    use super::*;

    #[test]
    fn main() {
        let input = include_str!("input.txt");
        let unique_chunk = get_end_index_of_first_unique_chunk(input, 14);
        dbg!("The first unique chunk is: {}", unique_chunk);
        print!("Finished")
    }

    #[test]
    #[test]
    fn check_inputs() {
        let inputs = vec![
            TestInput {
                input: "bvwbjplbgvbhsrlpgdmjqwftvncz",
                expected_index: 23,
            },
            TestInput {
                input: "nppdvjthqldpwncqszvftbrmjlhg",
                expected_index: 23,
            },
            TestInput {
                input: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                expected_index: 29,
            },
            TestInput {
                input: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
                expected_index: 26,
            },
        ];

        for input in &inputs {
            assert_eq!(
                get_position_of_input(&input, 14),
                Some(input.expected_index)
            );
        }
    }
}
