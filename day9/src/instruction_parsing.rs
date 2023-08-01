#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    Down,
    Left,
    Right,
    Up,
}

pub fn parse_line(line: &str) -> Vec<Instruction> {
    let segments: Vec<&str> = line.split(' ').collect();
    let dir_char = segments[0].chars().nth(0).unwrap();

    let direction = match dir_char {
        'U' => Instruction::Up,
        'D' => Instruction::Down,
        'L' => Instruction::Left,
        'R' => Instruction::Right,
        _ => panic!("Invalid direction"),
    };

    let count = segments[1].parse::<i32>().unwrap();

    let mut instructions = Vec::new();

    for _ in 0..count {
        instructions.push(direction.clone());
    }

    instructions
}

pub fn parse_instructions(str: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in str.lines() {
        instructions.push(parse_line(line));
    }
    let instructions_flattened: Vec<Instruction> =
        instructions.iter().flatten().map(|i| i.clone()).collect();

    instructions_flattened.clone()
}
