use std::str::Lines;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, PartialEq, Clone)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl TryFrom<Lines<'_>> for Stacks {
    type Error = color_eyre::Report;

    fn try_from(lines: Lines) -> Result<Self, Self::Error> {
        // Find first row where all chars are spaces or numbers
        let index_row = lines
            .clone()
            .enumerate()
            .find(|(_, line)| {
                line.chars()
                    .all(|c| c.is_ascii_digit() || c.is_ascii_whitespace())
            })
            .map(|(i, _)| i)
            .ok_or_else(|| color_eyre::eyre::eyre!("No index row found"))?;

        // Find the stack positions
        let stack_positions = lines
            .clone()
            .nth(index_row)
            .ok_or_else(|| color_eyre::eyre::eyre!("No index row found"))?
            .chars()
            .enumerate()
            .filter(|(_, c)| *c != ' ')
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        let mut stacks = vec![];
        for _ in stack_positions.iter() {
            stacks.push(vec![]);
        }

        for line in lines.into_iter() {
            for (i, position) in stack_positions.iter().enumerate() {
                let element = line.chars().nth(*position).unwrap();
                if element != ' ' && !element.is_ascii_digit() {
                    stacks[i].push(element);
                }
            }
        }

        stacks.iter_mut().for_each(|stack| stack.reverse());

        Ok(Self { stacks })
    }
}

impl Stacks {
    fn apply_instructions(
        self,
        instructions: Vec<Instruction>,
    ) -> Result<Self, color_eyre::Report> {
        let starting_state = self.clone();

        instructions
            .into_iter()
            .try_fold(starting_state, |stacks, instruction| {
                stacks.apply_instruction(instruction)
            })
    }

    fn apply_instruction(mut self, instruction: Instruction) -> Result<Self, color_eyre::Report> {
        for _ in 0..instruction.n {
            let element_to_move = self.stacks[instruction.from - 1]
                .pop()
                .ok_or_else(|| color_eyre::eyre::eyre!("No element to move"))?;

            self.stacks[instruction.to - 1].push(element_to_move);
        }

        Ok(self.clone())
    }

    fn get_top_elements(&self) -> Vec<char> {
        self.stacks
            .iter()
            .map(|stack| stack.last().cloned().unwrap_or(' '))
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    n: usize,
    from: usize,
    to: usize,
}

impl TryFrom<&str> for Instruction {
    type Error = color_eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split_ascii_whitespace().collect::<Vec<_>>();

        let instructions = Self {
            n: parts[1].parse()?,
            from: parts[3].parse()?,
            to: parts[5].parse()?,
        };

        Ok(instructions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_instructions() {
        let stacks = Stacks {
            stacks: vec![vec!['C', 'M'], vec!['C'], vec!['P', 'P']],
        };

        let instructions = vec![
            Instruction {
                n: 1,
                from: 2,
                to: 3,
            },
            Instruction {
                n: 2,
                from: 1,
                to: 3,
            },
        ];

        let result = stacks.apply_instructions(instructions).unwrap();
        let expected = Stacks {
            stacks: vec![vec![], vec![], vec!['P', 'P', 'C', 'M', 'C']],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_starting_state_parsing() {
        let starting_state_input = "[C]     [P]\n[N] [C] [P]\n 1   2   3  ".lines();
        let starting_stacks = Stacks::try_from(starting_state_input).unwrap();
        assert_eq!(
            starting_stacks.stacks,
            vec![vec!['N', 'C'], vec!['C'], vec!['P', 'P']]
        );
    }

    #[test]
    fn test_instruction_parsing() {
        let instructions = "move 1 from 2 to 3\nmove 2 from 2 to 3\nmove 3 from 2 to 4\n"
            .lines()
            .map(|line| Instruction::try_from(line).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(
            instructions,
            vec![
                Instruction {
                    n: 1,
                    from: 2,
                    to: 3
                },
                Instruction {
                    n: 2,
                    from: 2,
                    to: 3
                },
                Instruction {
                    n: 3,
                    from: 2,
                    to: 4
                },
            ]
        )
    }

    #[test]
    fn starting_example() {
        let starting_input = include_str!("starting_input.txt");

        let result = main(starting_input);

        let top_elements_end_state = vec!['C', 'M', 'Z'];
        assert_eq!(result, top_elements_end_state);
    }

    #[test]
    fn final_result() {
        let input = include_str!("input.txt");
        let result = main(input);
        dbg!(result);
    }
}

fn main(starting_input: &str) -> Vec<char> {
    // Collect lines untill the first empty line
    let binding = starting_input
        .lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    let starting_state_lines = binding.lines();

    // Parse the starting state
    let starting_state = Stacks::try_from(starting_state_lines).unwrap();

    // Find all lines that start with "move"
    let instructions_lines = starting_input
        .lines()
        .skip_while(|line| !line.starts_with("move"))
        .collect::<Vec<_>>();

    // Parse the instructions
    let instructions = instructions_lines
        .iter()
        .map(|line| Instruction::try_from(line.clone()).unwrap())
        .collect::<Vec<_>>();

    // Create a new set of stacks
    let result = starting_state.apply_instructions(instructions).unwrap();
    result.get_top_elements()
}
