use std::any::type_name;
fn main() {
    // Load file in day1.txt
    let input = include_str!("day1.txt");

    println!("Input was {}", input);

    // Initialise a vector of i32
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

    // Sort the list by calories
    let fattest_elves = &elf_cals.sort();

    let top_4_cals = fattest_elves.take(4).sum();

    println!("{:?}", elf_cals);
    println!("Top 4 cals {}", top_4_cals);
}
