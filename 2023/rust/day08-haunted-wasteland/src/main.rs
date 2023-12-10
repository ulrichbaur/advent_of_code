use std::{collections::HashMap, u64};

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
// - Walk all nodes ending with A to nodes ending with Z at the same time
// - Count steps
fn main() {
    let input = include_str!("../data/input.txt").lines().collect();
    let result1 = solve1(&input);
    println!("Result Part 1: {}", result1);

    let result2 = solve2_optimized(&input);
    println!("Result Part 2: {}", result2);
}

fn solve1(input: &Vec<&str>) -> u32 {
    let mut instructions = Instructions::from(input[0]);

    let mut next_location: String = String::from("AAA");
    let goal = "ZZZ";
    let mut steps = 0;

    while next_location != goal {
        let line = search_for_line_in_input_starting_with(input, &next_location);
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

fn solve2_brute_force(input: &Vec<&str>) -> u32 {
    let mut instructions = Instructions::from(input[0]);

    let mut current_locations: Vec<String> = find_all_starting_positions(input, 'A');
    let traveling_node_count = current_locations.len();
    println!("Node Count: {}", traveling_node_count);

    let goal = "Z";
    let mut steps = 0;

    let mut nodes_at_goal: Vec<bool> = vec![false; traveling_node_count];

    while nodes_at_goal.iter().any(|&n| !n) {
        if steps % 1_000_000 == 0 {
            println!("Step {}", steps);
        }
        let next_instruction = instructions.next();
        for i in 0..traveling_node_count {
            let line = search_for_line_in_input_starting_with(input, &current_locations[i]);
            let node = Node::from(&line);
            current_locations[i] = match next_instruction {
                Instruction::Left => node.left,
                Instruction::Right => node.right,
            };

            if current_locations[i].ends_with(goal) {
                nodes_at_goal[i] = true;
            } else {
                nodes_at_goal[i] = false;
            }
        }
        steps += 1;
    }

    steps
}

// Optimizations:
// - build hashmap of nodes instead of creating nodes on the fly while walking
// - notice that step count between end points stay the same for every walking node -> lcm
fn solve2_optimized(input: &Vec<&str>) -> u64 {
    let (mut instructions, nodes) = parse_input(input);

    let mut current_nodes: Vec<String> = nodes
        .iter()
        .filter_map(|(&ref s, _v)| {
            if s.chars().nth(2) == Some('A') {
                Some(s.to_string())
            } else {
                None
            }
        })
        .collect();

    let traveling_node_count = current_nodes.len();
    println!("Node Count: {}", traveling_node_count);

    let goal = "Z";
    let mut steps = 0;

    let mut steps_to_end: Vec<u64> = vec![0; traveling_node_count];
    let mut nodes_at_goal: Vec<bool> = vec![false; traveling_node_count];

    while nodes_at_goal.iter().any(|&n| !n) {
        let next_instruction = instructions.next();
        for i in 0..traveling_node_count {
            let node: &String = &current_nodes[i];
            current_nodes[i] = match next_instruction {
                Instruction::Left => nodes[node].left.clone(),
                Instruction::Right => nodes[node].right.clone(),
            };

            if current_nodes[i].ends_with(goal) {
                nodes_at_goal[i] = true;
                steps_to_end[i] = steps + 1;
            }
        }
        steps += 1;
    }

    dbg!(&steps_to_end);

    steps_to_end
        .iter()
        .fold(steps_to_end[0], |acc, &x| least_common_multiplier(acc, x))
}

fn least_common_multiplier(a: u64, b: u64) -> u64 {
    (a * b) / greatest_common_divider(a, b)
}

fn greatest_common_divider(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let temp = a;
        a = b;
        b = temp % a;
    }

    a
}

fn parse_input(input: &Vec<&str>) -> (Instructions, HashMap<String, Node>) {
    let instructions = Instructions::from(input[0]);

    let mut nodes: HashMap<String, Node> = HashMap::new();
    for line in input.iter().skip(2) {
        let node = Node::from(line);
        let node_name = node.name.to_string();

        nodes.insert(node_name, node);
    }

    (instructions, nodes)
}

fn find_all_starting_positions(input: &Vec<&str>, search_term: char) -> Vec<String> {
    input
        .iter()
        .skip(2)
        .filter(|s| s.chars().nth(2) == Some(search_term))
        .map(|s| s.to_string())
        .collect()
}

fn search_for_line_in_input_starting_with<'a>(input: &Vec<&'a str>, search_term: &str) -> &'a str {
    input
        .iter()
        .find(|s| s.starts_with(search_term))
        .unwrap_or(&"")
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
    fn new(name: String, left: String, right: String) -> Node {
        Node { name, left, right }
    }

    fn from(input: &str) -> Node {
        let terms: Vec<&str> = input.split('=').map(|s| s.trim()).collect();

        let node_name = terms[0];

        let next_terms: Vec<&str> = terms[1].split(',').map(|s| s.trim()).collect();

        let left = next_terms[0].split('(').collect::<Vec<&str>>()[1];
        let right = next_terms[1].split(')').collect::<Vec<&str>>()[0];

        Node::new(node_name.to_string(), left.to_string(), right.to_string())
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

        assert_eq!(
            node,
            Node::new(
                String::from("AAA"),
                String::from("BBB"),
                String::from("CCC")
            )
        )
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
    fn can_solve_part2_for_sample_input() {
        let input = include_str!("../data/sample_input3.txt").lines().collect();

        let result = solve2_optimized(&input);

        assert_eq!(result, 6);
    }

    #[test]
    fn can_solve_part2_for_actual_input() {
        let input = include_str!("../data/input.txt").lines().collect();

        let result = solve2_optimized(&input);

        assert_eq!(result, 21165830176709);
    }
}
