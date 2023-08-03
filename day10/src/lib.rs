#[derive(Clone, Copy)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    pub fn parse(i: &str) -> Instruction {
        let instruction_type = i.split(' ').nth(0).unwrap();

        if instruction_type == "addx" {
            let amount = i.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
            return Instruction::Addx(amount);
        }

        Instruction::Noop
    }

    pub fn cycle_count(&self) -> i32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

struct InstructionHistory {
    instructions: Vec<Instruction>,
}

impl InstructionHistory {
    pub fn register_value_during_cycle(&self, cycle: &i32) -> i32 {
        let mut register_value = 1;
        let mut cycle_count = 1;

        for instruction in self.instructions.iter() {
            cycle_count += instruction.cycle_count();
            if cycle_count > *cycle {
                break;
            }

            match instruction {
                Instruction::Noop => {}
                Instruction::Addx(amount) => register_value += amount,
            }
        }

        register_value
    }
}

pub fn main(input: &str) {
    let instructions = input
        .lines()
        .map(|i| Instruction::parse(i))
        .collect::<Vec<_>>();

    let history = InstructionHistory {
        instructions: instructions.clone(),
    };

    let mut pixels = vec![];

    for cycle_count in 1..=240 {
        let register_value = history.register_value_during_cycle(&cycle_count);
        let crt_position = (cycle_count - 1) % 40;
        let sprite_interval = crt_position - 2..crt_position + 2;

        println!("Register value: {:?}", register_value);
        println!("CRT position: {:?}", crt_position);
        println!("Sprite interval: {:?}", sprite_interval);
        println!("\n");

        if sprite_interval.contains(&register_value) {
            pixels.push("#");
        } else {
            pixels.push(".");
        }

        if cycle_count.clone() % 40 == 0 {
            pixels.push("\n");
        }
    }

    for pixel in pixels {
        print!("{}", pixel);
    }
}

mod tests {
    use crate::main;

    #[test]
    fn test_main() {
        let input = include_str!("input.txt");
        main(input);
    }

    #[test]
    fn test_smaller_sample() {
        let input = include_str!("small_sample.txt");
        main(input);
    }

    #[test]
    fn test_sample_parsing() {
        let input = include_str!("sample_input.txt");
        let instructions = input
            .lines()
            .map(|i| super::Instruction::parse(i))
            .collect::<Vec<_>>();

        let n_cycles = instructions.iter().map(|i| i.cycle_count()).sum::<i32>();
        assert_eq!(n_cycles, 5);

        let history = super::InstructionHistory {
            instructions: instructions,
        };
        assert_eq!(history.register_value_during_cycle(&3), 1);
        assert_eq!(history.register_value_during_cycle(&4), 4);
        assert_eq!(history.register_value_during_cycle(&5), 4);
        assert_eq!(history.register_value_during_cycle(&6), -1);
    }
}
