#[cfg(test)]
mod main_tests {
    use day1_lib::{lib_main, load_input};

    #[test]
    fn main() {
        let input = load_input();
        lib_main(input)
    }
}
