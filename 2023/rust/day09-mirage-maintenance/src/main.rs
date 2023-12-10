// Task:
// Part 1:
// - each line contains history of a single value
// - prediction of the next value in each history
//   - difference at each step
//   - repeat if not all zeroes
//   - then, back fill the histories to extrapolate next value
// - sum up extrapolated values
// Part 2:
// - ...
fn main() {
    let input = include_str!("../data/input.txt").lines().collect();
    let result1 = solve1(&input);
    println!("Result Part 1: {}", result1);

    // let result2 = solve2(&input);
    // println!("Result Part 2: {}", result2);
}

fn solve1(input: &Vec<&str>) -> i64 {
    // for every history
    // -> parse history -> Vec<i64>
    // -> generate -> Vec<Vec<i64>>
    // -> extrapolate value

    let mut sum = 0;
    for line in input.iter() {
        let history = parse_history(line);

        let mut histories: Vec<Vec<i64>> = vec![];
        histories.push(history);

        let mut index = 0;
        while histories.last().unwrap().iter().any(|&v| v != 0) {
            let mut current_diff: Vec<i64> = vec![];
            for i in 0..histories.last().unwrap().len() - 1 {
                let diff = histories[index][i + 1] - histories[index][i];
                current_diff.push(diff);
            }
            histories.push(current_diff);
            index += 1;
        }
        for i in 0..histories.len() - 1 {
            let current_index = histories.len() - (i + 1);
            let new_value = *histories[current_index - 1].last().unwrap()
                + *histories[current_index].last().unwrap();
            histories[current_index - 1].push(new_value)
        }

        let value = *histories[0].last().unwrap();
        sum += value;
    }

    sum
}

fn solve2(input: &Vec<&str>) -> i64 {
    todo!()
}

fn parse_history(line: &str) -> Vec<i64> {
    line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1_for_sample_input() {
        let input = include_str!("../data/sample_input.txt").lines().collect();

        let result = solve1(&input);

        assert_eq!(result, 114);
    }

    #[test]
    fn can_solve_part1_for_actual_input() {
        let input = include_str!("../data/input.txt").lines().collect();

        let result = solve1(&input);

        assert_eq!(result, 1806615041);
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
