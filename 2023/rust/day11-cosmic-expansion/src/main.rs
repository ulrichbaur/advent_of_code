use std::collections::HashSet;

// Task:
// Part 1:
// - Shortest path between every pair of galaxies
// - Image contains empty space (.) and galaxies (#)
// - However space expands
//   - any row or colum without galaxies should be twice as big
// - for each pair (order doesn't matter)
//   - find shortest path between
//   - allowed to pass through other galaxies
// Part 2:
// - now empty space expands a lot further
// - each empty row or column gets replaced by a million empty rows or columns
fn main() {
    let input = include_str!("../data/input.txt").lines().collect();
    let grid = get_grid(&input);
    print_character_grid(&grid);

    let result1 = solve1(&grid);
    println!("Result Part 1: {}", result1);

    let result2 = solve2(&grid);
    println!("Result Part 2: {}", result2);
}

fn solve1(grid: &Grid2d<char>) -> i64 {
    solve_for_expansion(grid, 2)
}

fn solve2(grid: &Grid2d<char>) -> i64 {
    solve_for_expansion(grid, 1_000_000)
}

fn solve_for_expansion(grid: &Grid2d<char>, expansion_factor: i64) -> i64 {
    // find all galaxies
    let galaxy_positions = get_galaxy_positions(grid);

    // get empty rows & columns
    let (empty_rows, empty_columns) = get_empty_rows_and_columns(grid);

    let mut total_distance = 0;
    for i in 0..galaxy_positions.len() - 1 {
        let left_pos = galaxy_positions[i];

        for j in i + 1..galaxy_positions.len() {
            let right_pos = galaxy_positions[j];

            // calculate distance between positions
            let distance = i64::abs(left_pos.x - right_pos.x) + i64::abs(left_pos.y - right_pos.y);

            // count empty rows between positions
            let rows_between_pos = empty_rows
                .iter()
                .filter(|&row| {
                    &left_pos.y < row && row < &right_pos.y
                        || &right_pos.y < row && row < &left_pos.y
                })
                .count();
            // count empty columns between positions
            let columns_between_pos = empty_columns
                .iter()
                .filter(|&column| {
                    &left_pos.x < column && column < &right_pos.x
                        || &right_pos.x < column && column < &left_pos.x
                })
                .count();

            // distance of universe expansion
            let expansion_distance =
                (rows_between_pos as i64 + columns_between_pos as i64) * (expansion_factor - 1);

            total_distance += distance + expansion_distance;
        }
    }

    total_distance
}

fn get_grid(input: &Vec<&str>) -> Grid2d<char> {
    let height = input.len();
    let width = input[0].len();

    let mut grid: Grid2d<char> = Grid2d::new(width as i64, height as i64);
    for row_index in 0..height {
        for column_index in 0..width {
            let character = input[row_index].chars().nth(column_index).unwrap();
            grid.set(row_index as i64, column_index as i64, character);
        }
    }

    grid
}

fn print_character_grid(grid: &Grid2d<char>) {
    for row in 0..grid.height {
        for column in 0..grid.width {
            print!("{}", grid.at(row, column));
        }
        println!("");
    }
}
fn get_galaxy_positions(grid: &Grid2d<char>) -> Vec<Vector2d> {
    let mut galaxy_positions: Vec<Vector2d> = Vec::new();
    for i in 0..grid.size() {
        let c = grid.at_index(i);

        if c != '#' {
            continue;
        }

        let position = grid.get_coordinates_from_index(i);
        galaxy_positions.push(position);
    }
    galaxy_positions
}
fn get_empty_rows_and_columns(grid: &Grid2d<char>) -> (HashSet<i64>, HashSet<i64>) {
    let mut empty_rows: HashSet<i64> = HashSet::new();
    empty_rows.extend(0..grid.height);
    let mut empty_columns: HashSet<i64> = HashSet::new();
    empty_columns.extend(0..grid.width);
    for i in 0..grid.size() {
        let c = grid.at_index(i);

        if c != '#' {
            continue;
        }

        let position = grid.get_coordinates_from_index(i);
        empty_columns.remove(&position.x);
        empty_rows.remove(&position.y);
    }
    (empty_rows, empty_columns)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector2d {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Grid2d<T: Copy + Default> {
    width: i64,
    height: i64,
    data: Vec<T>,
}

impl<T: Copy + Default> Grid2d<T> {
    fn new(width: i64, height: i64) -> Grid2d<T> {
        Grid2d {
            width,
            height,
            data: vec![Default::default(); width as usize * height as usize],
        }
    }
    fn is_in_bounds(self: &Self, x: i64, y: i64) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }
    fn is_position_in_bounds(self: &Self, position: Vector2d) -> bool {
        position.x >= 0 && position.x < self.width && position.y >= 0 && position.y < self.height
    }

    fn at(self: &Self, x: i64, y: i64) -> T {
        let index = y as usize * self.width as usize + x as usize;
        return self.data[index];
    }

    fn at_position(self: &Self, position: Vector2d) -> T {
        let index = self.get_index_from_coordinates(position.x, position.y);
        return self.data[index as usize];
    }

    fn at_index(self: &Self, index: i64) -> T {
        return self.data[index as usize];
    }

    fn set(self: &mut Self, x: i64, y: i64, value: T) {
        let index = y as usize * self.width as usize + x as usize;
        self.data[index] = value;
    }

    fn size(self: &Self) -> i64 {
        self.width * self.height
    }

    fn get_coordinates_from_index(self: &Self, index: i64) -> Vector2d {
        Vector2d {
            x: index / self.width,
            y: index % self.width,
        }
    }

    fn get_index_from_coordinates(self: &Self, x: i64, y: i64) -> i64 {
        y * self.width + x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1_for_sample_input() {
        let input = include_str!("../data/sample_input.txt").lines().collect();
        let grid = get_grid(&input);

        let result = solve1(&grid);

        assert_eq!(result, 374);
    }

    #[test]
    fn can_solve_part1_for_actual_input() {
        let input = include_str!("../data/input.txt").lines().collect();
        let grid = get_grid(&input);

        let result = solve1(&grid);

        assert_eq!(result, 9565386);
    }

    #[test]
    fn can_solve_part2_for_sample_input() {
        let input = include_str!("../data/sample_input.txt").lines().collect();
        let grid = get_grid(&input);

        let result = solve2(&grid);

        assert_eq!(result, 82000210);
    }

    #[test]
    fn can_solve_part2_for_actual_input() {
        let input = include_str!("../data/input.txt").lines().collect();
        let grid = get_grid(&input);

        let result = solve2(&grid);

        assert_eq!(result, 857986849428);
    }
}
