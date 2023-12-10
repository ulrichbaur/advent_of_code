// Task:
// Part 1:
// - Walk from AAA to ZZZ using the given left/right instructions
// - Notation:
//    - Left-Right instructions:
//      - Example: LRRL
//      - repeat instruction if not at ZZZ after run out of instructions
//    - Network Nodes:
//      - Example: UUU = (VVV,WWW)
//      - defines node UUU with left destination VVV and right destination WWW
//  - Count steps required to walk from AAA to ZZZ
// Part 2:
// - ...
fn main() {
    let input = include_str!("../data/input.txt").lines().collect();
    let result1 = solve1(&input);
    println!("Result Part 1: {}", result1);

    // let result2 = solve2(&input);
    // println!("Result Part 2: {}", result2);
}

fn solve1(input: &Vec<&str>) -> u32 {
    let mut instructions = Instructions::from(input[0]);

    let mut next_location: String = String::from("AAA");
    let goal = "ZZZ";
    let mut steps = 0;

    while next_location != goal {
        let line = search_for_string(input, &next_location);
        let node = Node::from(&line);
        let next_instruction = instructions.next();
        match next_instruction {
            Instruction::Left => {

                next_location = node.left;
            }
            Instruction::Right => {
                next_location = node.right;
            }
        };
        steps += 1;
    }

    steps
}

fn solve2(input: &Vec<&str>) -> u32 {
    todo!()
}

fn search_for_string(input: &Vec<&str>, search_term: &str) -> String {
    input
        .iter()
        .find(|s| s.starts_with(search_term))
        .unwrap()
        .to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct Instructions {
    instructions: Vec<Instruction>,
    index: usize,
}

impl Instructions {
    fn new(instructions: Vec<Instruction>) -> Instructions {
        Instructions {
            instructions,
            index: 0,
        }
    }
    fn from(input: &str) -> Instructions {
        let mut instructions = vec![];
        for character in input.chars() {
            match character {
                'L' => instructions.push(Instruction::Left),
                'R' => instructions.push(Instruction::Right),
                _ => panic!("Unable to parse instruction from {}", character),
            }
        }
        Instructions::new(instructions)
    }

    fn next(self: &mut Self) -> Instruction {
        let current_index = self.index;
        self.index = (self.index + 1) % self.instructions.len();
        self.instructions[current_index]
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn new(name: &str, left: &str, right: &str) -> Node {
        Node {
            name: name.to_owned(),
            left: left.to_owned(),
            right: right.to_owned(),
        }
    }

    fn from(input: &str) -> Node {
        let terms: Vec<&str> = input.split('=').map(|s| s.trim()).collect();

        let node_name = terms[0];

        let next_terms: Vec<&str> = terms[1].split(',').map(|s| s.trim()).collect();

        let left = next_terms[0].split('(').collect::<Vec<&str>>()[1];
        let right = next_terms[1].split(')').collect::<Vec<&str>>()[0];

        Node::new(node_name, left, right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_instructions_from_string() {
        let input = "LRLRRRL";

        let instructions = Instructions::from(input);

        assert_eq!(
            instructions,
            Instructions::new(vec![
                Instruction::Left,
                Instruction::Right,
                Instruction::Left,
                Instruction::Right,
                Instruction::Right,
                Instruction::Right,
                Instruction::Left,
            ])
        )
    }

    #[test]
    fn can_parse_node_from_string() {
        let input = "AAA = (BBB, CCC)";

        let node = Node::from(input);

        assert_eq!(node, Node::new("AAA", "BBB", "CCC"))
    }

    #[test]
    fn can_solve_part1_for_sample_input() {
        let input = include_str!("../data/sample_input.txt").lines().collect();

        let result = solve1(&input);

        assert_eq!(result, 2);
    }

    #[test]
    fn can_solve_part1_for_actual_input() {
        let input = include_str!("../data/input.txt").lines().collect();

        let result = solve1(&input);

        assert_eq!(result, 21409);
    }

    #[test]
    #[ignore]
    fn can_solve_part2_for_sample_input() {
        let input = include_str!("../data/sample_input.txt").lines().collect();

        let result = solve2(&input);

        assert_eq!(result, 0);
    }

    #[test]
    #[ignore]
    fn can_solve_part2_for_actual_input() {
        let input = include_str!("../data/input.txt").lines().collect();

        let result = solve2(&input);

        assert_eq!(result, 0);
    }
}
