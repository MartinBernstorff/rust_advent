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

pub fn main(input: &str) -> Vec<i32> {
    let instructions = input
        .lines()
        .map(|i| Instruction::parse(i))
        .collect::<Vec<_>>();

    let history = InstructionHistory {
        instructions: instructions.clone(),
    };

    let cycle_counts = [20, 60, 100, 140, 180, 220];
    let counts_during_cycles = cycle_counts.map(|c| history.register_value_during_cycle(&c));

    let mut signal_strengths = vec![];

    for i in 0..cycle_counts.len() {
        let cycle_count = cycle_counts[i];
        let register_value = counts_during_cycles[i];
        let signal_strength = register_value * cycle_count;
        signal_strengths.push(signal_strength);

        println!("{} {}", cycle_count, signal_strength)
    }
    signal_strengths
}

mod tests {
    use crate::main;

    #[test]
    fn test_main() {
        let input = include_str!("input.txt");
        let signal_strenghts = main(input);

        let sum_of_signal_strengths = signal_strenghts.iter().sum::<i32>();
        println!("sum_of_signal_strengths: {}", sum_of_signal_strengths);
    }

    #[test]
    fn test_small_sample() {
        let signal_strengths = main(include_str!("small_sample.txt"));
        assert_eq!(signal_strengths, vec![420, 1140, 1800, 2940, 2880, 3960]);
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
