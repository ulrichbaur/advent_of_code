// Task:
// Part 1:
// - Find mirror line in patterns of ash (.) and rock (#)
// Part 2:
// - ...
fn main() {
    let input: Vec<&str> = include_str!("../data/input.txt").split("\n\n").collect();

    let now = std::time::Instant::now();
    let result1 = solve1(&input);
    println!("Result Part 1: {} ({:?})", result1, now.elapsed());

    // let now = std::time::Instant::now();
    // let result2 = solve2(&input);
    // println!("Result Part 2: {} ({:?})", result2, now.elapsed());
}

fn solve1(input: &[&str]) -> u32 {
    let mut sum = 0;

    // parse grid
    for grid_lines in input {
        let height = grid_lines.lines().count();
        let width = grid_lines.lines().collect::<Vec<_>>()[0].len();

        let grid = parse_grid(grid_lines);

        // find mirror line
        // - search in columns
        let mut columns: Vec<String> = vec![];
        for i in 0..width {
            columns.push(grid.get_column(i as i64).iter().collect());
        }
        dbg!(&columns);

        let mirror_column = find_mirror(&columns);

        // - search in rows
        let mut rows: Vec<String> = vec![];
        for i in 0..height {
            rows.push(grid.get_row(i as i64).iter().collect());
        }
        dbg!(&rows);

        let mirror_row = find_mirror(&rows);

        if mirror_row != -1 {
            sum += mirror_row * 100;
        }
        if mirror_column != -1 {
            sum += mirror_column;
        }
        dbg!(sum);
    }

    sum as u32
}

fn solve2(_input: &[&str]) -> u32 {
    todo!()
}

fn find_mirror(input: &Vec<String>) -> i64 {
    let max = input.len();

    for i in 1..max {
        if is_mirrored(&input[0..i], &input[i..max]) {
            return i as i64;
        }
    }
    -1
}

fn is_mirrored(a: &[String], b: &[String]) -> bool {
    let a_len = a.len();
    let b_len = b.len();
    let min = a_len.min(b_len);
    for i in 0..min {
        if a[a_len - i - 1] != b[i] {
            return false;
        }
    }
    true
}

fn parse_grid(input: &str) -> Grid2d<char> {
    let height = input.lines().count();
    let width = input.lines().collect::<Vec<_>>()[0].len();

    let mut grid: Grid2d<char> = Grid2d::new(width as i64, height as i64);
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            grid.set(x as i64, y as i64, char);
        }
    }
    grid
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector2d {
    x: i64,
    y: i64,
}

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
    fn is_in_bounds(&self, x: i64, y: i64) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }
    fn is_position_in_bounds(&self, position: Vector2d) -> bool {
        position.x >= 0 && position.x < self.width && position.y >= 0 && position.y < self.height
    }

    fn at(&self, x: i64, y: i64) -> T {
        let index = y as usize * self.width as usize + x as usize;
        self.data[index]
    }

    fn at_position(&self, position: Vector2d) -> T {
        let index = self.get_index_from_coordinates(position.x, position.y);
        self.data[index as usize]
    }

    fn at_index(&self, index: i64) -> T {
        self.data[index as usize]
    }

    fn set(&mut self, x: i64, y: i64, value: T) {
        let index = y as usize * self.width as usize + x as usize;
        self.data[index] = value;
    }

    fn size(&self) -> i64 {
        self.width * self.height
    }

    fn get_coordinates_from_index(&self, index: i64) -> Vector2d {
        Vector2d {
            x: index / self.width,
            y: index % self.width,
        }
    }

    fn get_index_from_coordinates(&self, x: i64, y: i64) -> i64 {
        y * self.width + x
    }

    fn get_row(&self, row_index: i64) -> Vec<T> {
        let mut vec = Vec::new();
        for i in 0..self.width {
            vec.push(self.at(i, row_index));
        }
        vec
    }

    fn get_column(&self, column_index: i64) -> Vec<T> {
        let mut vec = Vec::new();
        for i in 0..self.height {
            vec.push(self.at(column_index, i));
        }
        vec
    }
}

fn print_character_grid(grid: &Grid2d<char>) {
    println!();
    for row in 0..grid.height {
        for column in 0..grid.width {
            print!("{}", grid.at(column, row));
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1_for_sample_input() {
        let input: Vec<&str> = include_str!("../data/sample_input.txt")
            .split("\n\n")
            .collect();

        let result = solve1(&input);

        assert_eq!(result, 405);
    }

    #[test]
    #[ignore]
    fn can_solve_part1_for_actual_input() {
        let input: Vec<&str> = include_str!("../data/input.txt").split("\n\n").collect();

        let result = solve1(&input);

        assert_eq!(result, 0);
    }

    #[test]
    #[ignore]
    fn can_solve_part2_for_sample_input() {
        let input: Vec<&str> = include_str!("../data/sample_input.txt")
            .split("\n\n")
            .collect();

        let result = solve2(&input);

        assert_eq!(result, 0);
    }

    #[test]
    #[ignore]
    fn can_solve_part2_for_actual_input() {
        let input: Vec<&str> = include_str!("../data/input.txt").split("\n\n").collect();

        let result = solve2(&input);

        assert_eq!(result, 0);
    }
}
