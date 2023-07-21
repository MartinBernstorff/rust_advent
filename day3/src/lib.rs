pub fn lib_main(input: &'static str) {
    let total_priorities: u32 = input
        .lines()
        .map(split_str_to_compartments)
        .filter_map(|rucksack| get_shared_char(rucksack.compartment_one, rucksack.compartment_two))
        .map(get_char_priority)
        .sum();

    println!("{}", total_priorities);
}

pub fn load_input() -> &'static str {
    include_str!("day3.txt")
}

pub fn get_shared_char<'a>(input_1: &'a str, input_2: &'a str) -> Option<char> {
    for char1 in input_1.chars() {
        for char2 in input_2.chars() {
            if char1 == char2 {
                return Some(char1);
            }
        }
    }

    None
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
        let input = Rucksack {
            compartment_one: "AA",
            compartment_two: "BBAA",
        };

        let intersect = get_shared_char(input.compartment_one, input.compartment_two);

        assert_eq!(intersect, Some('A'));
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
}
