use std::collections::VecDeque;

// Task:
// Part 1:
// - Find point in loop that is farthest from the starting position
// - pipe is one large, continuous loop
// - Notation:
//   - '|' vertical pipe connecting north and south
//   - '-' horizontal pipe connecting east and west
//   - 'L' 90-degree bend connecting north and east
//   - 'J' 90-degree bend connecting north and west
//   - '7' 90-degree bend connecting south and west
//   - 'F' 90-degree bend connecting south and east
//   - '.' ground; no pipe
//   - 'S' starting position; pipe but sketch doesn't show direction
// Part 2:
// - Calculate the area enclosed by the loop
// - Water can squeeze through pipes; it needs to be fully enclosed
fn main() {
    let input = include_str!("../data/input.txt").lines().collect();
    let result1 = solve1(&input);
    println!("Result Part 1: {}", result1);

    let result2 = solve2(&input);
    println!("Result Part 2: {}", result2);
}

fn solve1(input: &Vec<&str>) -> i32 {
    let mut grid = get_grid(input);

    // determine start position and pipe type
    let start_position = get_starting_pipe(&mut grid);
    print_grid(&grid);

    // BFS to find the distance
    let mut distances: Vec<Vec<i32>> = vec![vec![-1; input[0].len()]; input.len()];
    distances[start_position.0 as usize][start_position.1 as usize] = 0;

    // init explore loop
    let mut exploration_queue: VecDeque<(Vector2D, Vector2D)> = VecDeque::new();
    let connected_pipes = get_connected_pipes(&grid, start_position);
    for pipe in connected_pipes {
        exploration_queue.push_back((start_position, pipe));
    }

    // explore loop
    while !exploration_queue.is_empty() {
        let (from, to) = exploration_queue.pop_back().unwrap();

        if distances[to.0 as usize][to.1 as usize] != -1 {
            continue;
        }
        distances[to.0 as usize][to.1 as usize] = distances[from.0 as usize][from.1 as usize] + 1;
        let connected_pipes = get_connected_pipes(&grid, to);
        for pipe in connected_pipes {
            exploration_queue.push_front((to, pipe));
        }
    }

    // calculate max distance
    distances.iter().fold(std::i32::MIN, |a, b| {
        a.max(b.iter().fold(std::i32::MIN, |c, d| c.max(*d)))
    })
}

fn solve2(input: &Vec<&str>) -> i32 {
    let mut grid = get_grid(input);

    // determine start position and pipe type
    let start_position = get_starting_pipe(&mut grid);
    print_grid(&grid);

    // BFS to find the distance
    let mut distances: Vec<Vec<i32>> = vec![vec![-1; input[0].len()]; input.len()];
    distances[start_position.0 as usize][start_position.1 as usize] = 0;

    // init explore loop
    let mut exploration_queue: VecDeque<(Vector2D, Vector2D)> = VecDeque::new();
    let connected_pipes = get_connected_pipes(&grid, start_position);
    for pipe in connected_pipes {
        exploration_queue.push_back((start_position, pipe));
    }

    // explore loop
    while !exploration_queue.is_empty() {
        let (from, to) = exploration_queue.pop_back().unwrap();

        if distances[to.0 as usize][to.1 as usize] != -1 {
            continue;
        }
        distances[to.0 as usize][to.1 as usize] = distances[from.0 as usize][from.1 as usize] + 1;
        let connected_pipes = get_connected_pipes(&grid, to);
        for pipe in connected_pipes {
            exploration_queue.push_front((to, pipe));
        }
    }

    // replace pipes not in loop with ground
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c = grid[i][j];
            if c != '.' && distances[i][j] == -1 {
                grid[i][j] = '.';
            }
        }
    }
    print_grid(&grid);

    // create big grid that's double the size of the original to make pathfinding easier between pipes`
    let mut big_grid: Vec<Vec<char>> = vec![vec![' '; grid[0].len() * 2]; grid.len() * 2];
    let big_width = big_grid[0].len();
    let big_height = big_grid.len();

    // init top, bottom, left, and right with empty space
    for i in 0..big_width {
        big_grid[i][0] = '0';
        big_grid[i][big_height - 1] = '0';
    }
    for i in 0..big_height {
        big_grid[0][i] = '0';
        big_grid[big_width - 1][i] = '0';
    }

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let c = grid[row][col];

            // set pipes to original value
            if c != '.' && distances[row][col] != -1 {
                big_grid[row * 2][col * 2] = c;
                continue;
            }

            // if it's on edge, already set to 0
            if row == 0 || row == grid.len() - 1 || col == 0 || col == grid[0].len() - 1 {
                continue;
            }

            // replace ground with I
            big_grid[row * 2][col * 2] = 'I';
        }
    }

    // walk through pipe west/east and north/south pairs and mark blocking connections with X
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if col < grid.len() - 1 {
                let west_pipe = grid[row][col];
                let east_pipe = grid[row][col + 1];
                match west_pipe {
                    '-' | 'L' | 'F' => match east_pipe {
                        '-' | 'J' | '7' => big_grid[row * 2][col * 2 + 1] = 'X',
                        _ => print!(""),
                    },
                    _ => print!(""),
                }
            }
            if row < grid.len() - 1 {
                let north_pipe = grid[row][col];
                let south_pipe = grid[row + 1][col];
                match north_pipe {
                    '|' | '7' | 'F' => match south_pipe {
                        '|' | 'J' | 'L' => big_grid[row * 2 + 1][col * 2] = 'X',
                        _ => print!(""),
                    },
                    _ => print!(""),
                }
            }
        }
    }

    let big_bounds = Vector2D(big_height as i32, big_width as i32);
    // from each 0, flood any Is or empty space with 0s
    for row in 0..big_height {
        for col in 0..big_width {
            let c = big_grid[row][col];

            // set pipes to original value
            if c != '0' {
                continue;
            }

            // depth first flood fill
            let current_position = Vector2D(row as i32, col as i32);
            let mut stack: VecDeque<Vector2D> = VecDeque::new();
            stack.push_back(add_positions(&current_position, &UP));
            stack.push_back(add_positions(&current_position, &LEFT));
            stack.push_back(add_positions(&current_position, &DOWN));
            stack.push_back(add_positions(&current_position, &RIGHT));

            while !stack.is_empty() {
                let next_position = stack.pop_back().unwrap();

                // skip if not in bounds
                if !position_is_in_bounds(&big_bounds, &next_position) {
                    continue;
                }

                // skip if already blocking
                let c = big_grid[next_position.0 as usize][next_position.1 as usize];
                if c != ' ' && c != 'I' {
                    continue;
                }
                big_grid[next_position.0 as usize][next_position.1 as usize] = '0';

                // continue exploring

                stack.push_back(add_positions(&next_position, &UP));
                stack.push_back(add_positions(&next_position, &LEFT));
                stack.push_back(add_positions(&next_position, &DOWN));
                stack.push_back(add_positions(&next_position, &RIGHT));
            }
        }
    }

    big_grid.iter().flatten().filter(|&c| *c == 'I').count() as i32
}

fn get_grid(input: &Vec<&str>) -> Vec<Vec<char>> {
    let height = input.len();
    let width = input[0].len();

    let mut grid = vec![vec!['.'; width]; height];
    for row_index in 0..height {
        for column_index in 0..width {
            let character = input[row_index].chars().nth(column_index).unwrap();
            grid[row_index][column_index] = character;
        }
    }

    grid
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        for character in line {
            print!("{}", character);
        }
        println!("");
    }
}

fn get_connected_pipes(grid: &Vec<Vec<char>>, position: Vector2D) -> Vec<Vector2D> {
    let mut candidate_pipes = vec![Vector2D(-3, -3), Vector2D(-3, -3)];

    let c = grid[position.0 as usize][position.1 as usize];
    match c {
        '|' => {
            candidate_pipes[0] = add_positions(&position, &UP);
            candidate_pipes[1] = add_positions(&position, &DOWN);
        }
        '-' => {
            candidate_pipes[0] = add_positions(&position, &LEFT);
            candidate_pipes[1] = add_positions(&position, &RIGHT);
        }
        'L' => {
            candidate_pipes[0] = add_positions(&position, &UP);
            candidate_pipes[1] = add_positions(&position, &RIGHT);
        }
        'J' => {
            candidate_pipes[0] = add_positions(&position, &UP);
            candidate_pipes[1] = add_positions(&position, &LEFT);
        }
        '7' => {
            candidate_pipes[0] = add_positions(&position, &DOWN);
            candidate_pipes[1] = add_positions(&position, &LEFT);
        }
        'F' => {
            candidate_pipes[0] = add_positions(&position, &DOWN);
            candidate_pipes[1] = add_positions(&position, &RIGHT);
        }
        _ => panic!("connected: Unknown pipe type: {}", c),
    }

    let mut connected_pipes: Vec<Vector2D> = vec![];
    let bounds = Vector2D(grid.len() as i32, grid[0].len() as i32);
    for candidate in candidate_pipes {
        if position_is_in_bounds(&bounds, &candidate) {
            connected_pipes.push(candidate);
        }
    }

    connected_pipes
}

fn position_is_in_bounds(bounds: &Vector2D, position: &Vector2D) -> bool {
    if position.0 < 0 || position.1 < 0 {
        return false;
    }
    if position.0 >= bounds.0 || position.1 >= bounds.1 {
        return false;
    }
    true
}

fn get_starting_position(input: &Vec<Vec<char>>) -> Vector2D {
    let height = input.len();
    let width = input[0].len();

    for row_index in 0..height {
        for column_index in 0..width {
            let character = input[row_index][column_index];
            if character == 'S' {
                return Vector2D(row_index as i32, column_index as i32);
            }
        }
    }
    panic!("No starting point in input data")
}

fn get_starting_pipe(grid: &mut Vec<Vec<char>>) -> Vector2D {
    let start = get_starting_position(grid);
    let pipe_types = ['|', '-', 'L', 'J', '7', 'F'];

    let mut valid_pipe;
    'outer: for pipe_type in pipe_types {
        grid[start.0 as usize][start.1 as usize] = pipe_type;

        let connected_pipes = get_connected_pipes(&grid, start);

        valid_pipe = connected_pipes.len() == 2;
        if !valid_pipe {
            continue;
        }

        for connected_pipe in connected_pipes {
            let c = grid[connected_pipe.0 as usize][connected_pipe.1 as usize];
            match c {
                '|' => {
                    if connected_pipe != add_positions(&start, &UP)
                        && connected_pipe != add_positions(&start, &DOWN)
                    {
                        valid_pipe = false;
                    }
                }
                '-' => {
                    if connected_pipe != add_positions(&start, &RIGHT)
                        && connected_pipe != add_positions(&start, &LEFT)
                    {
                        valid_pipe = false;
                    }
                }
                'L' => {
                    if connected_pipe != add_positions(&start, &DOWN)
                        && connected_pipe != add_positions(&start, &LEFT)
                    {
                        valid_pipe = false;
                    }
                }
                'J' => {
                    if connected_pipe != add_positions(&start, &DOWN)
                        && connected_pipe != add_positions(&start, &RIGHT)
                    {
                        valid_pipe = false;
                    }
                }
                '7' => {
                    if connected_pipe != add_positions(&start, &UP)
                        && connected_pipe != add_positions(&start, &RIGHT)
                    {
                        valid_pipe = false;
                    }
                }
                'F' => {
                    if connected_pipe != add_positions(&start, &UP)
                        && connected_pipe != add_positions(&start, &LEFT)
                    {
                        valid_pipe = false;
                    }
                }
                '.' => valid_pipe = false,
                _ => panic!("starting: Unknown pipe type: {}", c),
            }
        }
        if valid_pipe {
            println!("Found pipe type: {}", pipe_type);
            break 'outer;
        }
    }
    start
}

const UP: Vector2D = Vector2D(-1, 0);
const DOWN: Vector2D = Vector2D(1, 0);
const LEFT: Vector2D = Vector2D(0, -1);
const RIGHT: Vector2D = Vector2D(0, 1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector2D(i32, i32);

fn add_positions(a: &Vector2D, b: &Vector2D) -> Vector2D {
    Vector2D(a.0 + b.0, a.1 + b.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1_for_sample_input_1() {
        let input = include_str!("../data/sample_input_1.txt").lines().collect();

        let result = solve1(&input);

        assert_eq!(result, 4);
    }

    #[test]
    fn can_solve_part1_for_sample_input_2() {
        let input = include_str!("../data/sample_input_2.txt").lines().collect();

        let result = solve1(&input);

        assert_eq!(result, 4);
    }

    #[test]
    fn can_solve_part1_for_sample_input_3() {
        let input = include_str!("../data/sample_input_3.txt").lines().collect();

        let result = solve1(&input);

        assert_eq!(result, 8);
    }

    #[test]
    fn can_solve_part1_for_sample_input_4() {
        let input = include_str!("../data/sample_input_4.txt").lines().collect();

        let result = solve1(&input);

        assert_eq!(result, 8);
    }

    #[test]
    fn can_solve_part1_for_actual_input() {
        let input = include_str!("../data/input.txt").lines().collect();

        let result = solve1(&input);

        assert_eq!(result, 7102);
    }

    #[test]
    fn can_solve_part2_for_sample_input_1() {
        let input = include_str!("../data/sample_input_1.txt").lines().collect();

        let result = solve2(&input);

        assert_eq!(result, 1);
    }

    #[test]
    fn can_solve_part2_for_sample_input_2() {
        let input = include_str!("../data/sample_input_2.txt").lines().collect();

        let result = solve2(&input);

        assert_eq!(result, 1);
    }

    #[test]
    fn can_solve_part2_for_sample_input_3() {
        let input = include_str!("../data/sample_input_3.txt").lines().collect();

        let result = solve2(&input);

        assert_eq!(result, 1);
    }

    #[test]
    fn can_solve_part2_for_sample_input_4() {
        let input = include_str!("../data/sample_input_4.txt").lines().collect();

        let result = solve2(&input);

        assert_eq!(result, 1);
    }

    #[test]
    fn can_solve_part2_for_actual_input() {
        let input = include_str!("../data/input.txt").lines().collect();

        let result = solve2(&input);

        assert_eq!(result, 363);
    }
}
