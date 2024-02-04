use std::collections::HashMap;

// Task 12: Hot Springs
// Part 1:
// - For each row
//   - count all of the different arrangements of operational and broken spings
//   - that meet the given criteria
// - Sum up counts
// - Notation:
//   - # -> broken, . -> operational, ? -> unknown
//   - criteria are contiguous blocks of damaged springs
//   - Example:
//     - .??..??...?##. 1,1,3
//     - 4 possible arrangements:
//       - .#...#....###.
//       - .#....#...###.
//       - ..#...#...###.
//       - ..#..#....###.
// Part 2:
// - lines are folded
// - repeat springs by 5, separated by ? spring
// - repeat block criteria by 5
use itertools::{repeat_n, Itertools};

fn main() {
    let input: Vec<&str> = include_str!("../data/input.txt").lines().collect();
    let now = std::time::Instant::now();
    let result1 = solve1_bruteforce(&input);
    println!(
        "Result Part 1 (bruteforce): {} ({:?})",
        result1,
        now.elapsed()
    );

    let now = std::time::Instant::now();
    let result1 = solve1_recursion(&input);
    println!("Result Part 1: {} ({:?})", result1, now.elapsed());

    let now = std::time::Instant::now();
    let result2 = solve2(&input);
    println!("Result Part 2: {} ({:?})", result2, now.elapsed());
}

fn solve1_bruteforce(input: &[&str]) -> usize {
    let mut total_solution_count = 0;

    // for every line
    for line in input {
        let puzzle = Puzzle::from(line);

        // count possible arrangements
        let mut solutions = 0;

        // brute force solve
        let candidates = puzzle.generate_permutations();

        for candidate in candidates {
            if puzzle.check_candidate(candidate) {
                solutions += 1;
            }
        }

        total_solution_count += solutions;
    }
    total_solution_count
}

fn solve1_recursion(input: &[&str]) -> usize {
    let mut total_solution_count = 0;

    let mut cache = HashMap::new();
    // for every line
    for line in input {
        let puzzle = Puzzle::from(line);

        let solutions = puzzle.solve(&mut cache);
        total_solution_count += solutions;
    }
    total_solution_count
}

fn solve2(input: &[&str]) -> usize {
    let mut total_solution_count = 0;

    let mut cache = HashMap::new();
    // for every line
    for line in input {
        let puzzle = Puzzle::from2(line);

        let solutions = puzzle.solve(&mut cache);
        total_solution_count += solutions;
    }
    total_solution_count
}

#[derive(Debug)]
struct Puzzle {
    unknown: usize,
    springs: Vec<Spring>,
    blocks: Vec<usize>,
}

impl Puzzle {
    fn from(line: &str) -> Puzzle {
        let parts: Vec<&str> = line.split(' ').collect();

        // parse springs
        let springs: Vec<Spring> = parts[0].chars().map(Spring::from).collect();

        // parse blocks
        let blocks: Vec<usize> = parts[1]
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        let unknown = springs.iter().filter(|&s| s == &Spring::Unknown).count();

        Puzzle {
            unknown,
            springs,
            blocks,
        }
    }

    fn from2(line: &str) -> Puzzle {
        let parts: Vec<&str> = line.split(' ').collect();

        // parse springs
        let springs: Vec<Spring> = Itertools::intersperse(
            std::iter::repeat_with(|| parts[0].to_string()).take(5),
            "?".to_string(),
        )
        .collect::<String>()
        .chars()
        .map(Spring::from)
        .collect();

        // parse blocks
        let blocks: Vec<usize> = parts[1]
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .repeat(5);

        let unknown = springs.iter().filter(|&s| s == &Spring::Unknown).count();

        Puzzle {
            unknown,
            springs,
            blocks,
        }
    }

    fn generate_permutations(&self) -> Vec<Vec<Spring>> {
        repeat_n(
            [Spring::Operational, Spring::Damaged].into_iter(),
            self.unknown,
        )
        .multi_cartesian_product()
        .collect()
    }

    fn check_candidate(&self, candidate: Vec<Spring>) -> bool {
        let mut candidate_iter = candidate.into_iter();
        let filled_candidate: Vec<Spring> = self
            .springs
            .iter()
            .map(|s| match s {
                Spring::Unknown => candidate_iter.next().unwrap(),
                value => *value,
            })
            .collect();
        let damaged_counts: Vec<usize> = filled_candidate
            .iter()
            .group_by(|&s| s == &Spring::Damaged)
            .into_iter()
            .filter_map(|(is_damaged, group)| is_damaged.then_some(group.into_iter().count()))
            .collect();
        damaged_counts == self.blocks
    }
    fn solve(&self, cache: &mut Cache) -> usize {
        Self::do_solve(&self.springs, &self.blocks, cache)
    }

    fn do_solve(springs: &[Spring], blocks: &[usize], cache: &mut Cache) -> usize {
        // check if solution is in cache
        if let Some(&v) = cache.get(&(springs.to_vec(), blocks.to_vec())) {
            return v;
        }

        // if blocks is empty, we have a solution if there is no damaged spring left
        if blocks.is_empty() {
            let solution = match springs.iter().any(|&s| s == Spring::Damaged) {
                true => 0,
                false => 1,
            };
            cache.insert((springs.to_vec(), blocks.to_vec()), solution);
            return solution;
        }

        // check if there are enough springs left to fit remaining blocks
        if springs.len() < blocks.iter().sum::<usize>() + blocks.len() - 1 {
            cache.insert((springs.to_vec(), blocks.to_vec()), 0);
            return 0;
        }

        // skip operational springs
        if springs[0] == Spring::Operational {
            let solutions = Self::do_solve(&springs[1..], blocks, cache);
            cache.insert((springs.to_vec(), blocks.to_vec()), solutions);
            return solutions;
        }

        let mut solutions = 0;
        // check if it's possible to put the current block in the current position
        let block = blocks[0];
        // there should not be an operational spring in the block
        let possible = springs[0..block].iter().all(|&s| s != Spring::Operational);
        // the next spring after the block should not be an operational spring either
        let at_end = springs.len() <= block;
        let next_not_damaged = springs.len() > block && springs[block] != Spring::Damaged;
        if possible && (at_end || next_not_damaged) {
            let end = (block + 1).min(springs.len());
            solutions = Self::do_solve(&springs[end..], &blocks[1..], cache);
        }

        // if current spring is unknown, we could choose not to use it
        if springs[0] == Spring::Unknown {
            solutions += Self::do_solve(&springs[1..], blocks, cache);
        }

        cache.insert((springs.to_vec(), blocks.to_vec()), solutions);
        solutions
    }
}

type Cache = HashMap<(Vec<Spring>, Vec<usize>), usize>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn from(character: char) -> Spring {
        match character {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("Unknown spring type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1_for_sample_input() {
        let input: Vec<&str> = include_str!("../data/sample_input.txt").lines().collect();

        let result = solve1_bruteforce(&input);

        assert_eq!(result, 21);
    }

    #[test]
    fn can_solve_part1_for_actual_input() {
        let input: Vec<&str> = include_str!("../data/input.txt").lines().collect();

        let result = solve1_recursion(&input);

        assert_eq!(result, 7753);
    }

    #[test]
    fn can_solve_part2_for_sample_input() {
        let input: Vec<&str> = include_str!("../data/sample_input.txt").lines().collect();

        let result = solve2(&input);

        assert_eq!(result, 525152);
    }

    #[test]
    fn can_solve_part2_for_actual_input() {
        let input: Vec<&str> = include_str!("../data/input.txt").lines().collect();

        let result = solve2(&input);

        assert_eq!(result, 280_382_734_828_319);
    }
}
