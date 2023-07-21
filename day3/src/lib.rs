pub fn lib_main(input: &'static str) {
    let groups = parse_input_to_groups(input);

    let mut priorities = Vec::new();

    for group in groups {
        let shared_chars = get_shared_chars(group);

        if shared_chars.len() != 1 {
            panic!("Expected exactly one shared char");
        }

        priorities.push(get_char_priority(shared_chars[0]));
    }

    println!("{}", priorities.iter().sum::<u32>());
}

pub fn load_input() -> &'static str {
    include_str!("day3.txt")
}

pub fn parse_input_to_groups(input: &str) -> Vec<Vec<&str>> {
    // Split input into groups of 3 lines
    let mut groups = Vec::new();
    let mut lines = input.lines();

    loop {
        let group = lines.by_ref().take(3).collect::<Vec<_>>();

        if group.len() == 3 {
            groups.push(group);
        } else {
            return groups;
        }
    }
}

pub fn get_shared_chars(input_strs: Vec<&str>) -> Vec<char> {
    // Get intersection of chars in all strings
    use std::collections::HashSet;

    let sets = input_strs
        .iter()
        .map(|str| str.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

    let mut intersection = sets[0].clone();

    for set in sets.iter().skip(1) {
        intersection = intersection.intersection(set).cloned().collect();
    }

    intersection.iter().cloned().collect::<Vec<_>>()
}

pub fn get_char_priority(char: char) -> u32 {
    // If char is uppercase, return the index of the lowercase version + 26
    if char.is_uppercase() {
        return get_char_priority(char.to_ascii_lowercase()) + 26;
    }

    ASCII_LOWER
        .iter()
        .position(|&c| c == char)
        .expect("Char not found in ASCII_LOWER") as u32
}

pub struct Rucksack<'a> {
    compartment_one: &'a str,
    compartment_two: &'a str,
}

pub fn split_str_to_compartments(input: &str) -> Rucksack {
    let str_length = input.len();

    if str_length % 2 != 0 {
        panic!("Input string must be of even length");
    }

    let half_length = str_length / 2;

    let (first_half, second_half) = input.split_at(half_length);

    Rucksack {
        compartment_one: first_half,
        compartment_two: second_half,
    }
}

// Offset by Å (1) to match numbering in assignment
static ASCII_LOWER: [char; 27] = [
    'Å', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_get_char_priority() {
        assert_eq!(get_char_priority('a'), 1);
        assert_eq!(get_char_priority('A'), 27);
    }

    #[test]
    fn should_get_intersecting_chars() {
        let str_one = "ABCE";
        let str_two = "CDE";
        let str_three = "EFGC";

        let intersect = get_shared_chars(vec![str_one, str_two, str_three]);

        assert_eq!(intersect, vec!['C', 'E']);
    }

    #[test]
    fn test_rucksack_parsing() {
        let input_str = "AABB";
        let rucksack = split_str_to_compartments(input_str);
        assert_eq!(rucksack.compartment_one, "AA");
        assert_eq!(rucksack.compartment_two, "BB");
    }

    #[test]
    #[should_panic(expected = "Input string must be of even length")]
    fn test_rucksack_incorrect_length() {
        let input_str = "AAB";
        split_str_to_compartments(input_str);
    }

    #[test]
    fn test_load_input() {
        let input = load_input();
        assert_eq!(
            input.lines().take(1).collect::<Vec<_>>()[0],
            "FqdWDFppHWhmwwzdjvjTRTznjdMv"
        );
    }

    #[test]
    fn test_parse_input_into_groups() {
        let groups = parse_input_to_groups("1\n2\n3\n4");
        assert_eq!(groups, vec![vec!["1", "2", "3"]]);
    }
}
