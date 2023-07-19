pub fn load_input_file() -> color_eyre::Result<String> {
    let input = std::fs::read_to_string("src/day1.txt")?;
    Ok(input)
}
pub fn parse_string_to_elves(input: &str) -> Vec<i32> {
    let mut elf_cals: Vec<i32> = Vec::new();
    let mut cur_cals: Vec<i32> = Vec::new();

    for r in input.lines() {
        if r.is_empty() {
            elf_cals.push(cur_cals.iter().sum());
            cur_cals.clear();
        } else {
            cur_cals.push(r.parse::<i32>().unwrap());
        }
    }
    elf_cals
}

pub fn get_sum_of_top_n(input: &mut Vec<i32>, top_n: usize) -> i32 {
    input.sort();

    // Sort the list by calories
    return input.iter().rev().take(top_n).sum();
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
        let mut input = vec![1, 2, 3];
        let actual = super::get_sum_of_top_n(&mut input, 2);
        let expected = 5;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_gets_sum_of_n() {}
}
