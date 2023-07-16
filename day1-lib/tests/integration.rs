#[cfg(test)]
mod main_tests {
    use day1_lib::{get_sum_of_top_n, load_input_file, parse_string_to_elves};

    #[test]
    fn main() {
        // Load file in day1.txt
        let input = load_input_file();
        let mut parsed_elves = parse_string_to_elves(input);
        let combined_cals_from_richest_elves = get_sum_of_top_n(&mut parsed_elves, 4);

        // Write result to day1_result.txt
        println!(
            "The sum of the calories of the top 4 richest elves is {}",
            combined_cals_from_richest_elves
        );
    }
}
