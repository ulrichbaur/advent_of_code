// Task:
// Part 1:
// - add up any part numbers adjacent to a symbol, even diagonally
// - periods (.) do not count as a symbol
// Part 2:
// - if the symbol ('*') touches exactly two part numbers
// - multiply two part numbers
// - calculate sum

fn main() {
    let input: Vec<&str> = include_str!("../data/input.txt").lines().collect();
    let sum1 = solve1(&input);
    let sum2 = solve2(&input);
    println!("Sum for Part 1: {}", sum1);
    println!("Sum for Part 2: {}", sum2);
}

fn solve1(input: &Vec<&str>) -> u32 {
    let mut row_index = 0;

    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for line in input {
        let mut new_part_numbers = get_part_numbers(line, row_index);
        part_numbers.append(&mut new_part_numbers);
        let mut new_symbols = get_symbols(line, row_index);
        symbols.append(&mut new_symbols);
        row_index += 1;
    }

    let mut sum = 0;
    for part_number in part_numbers {
        for symbol in &symbols {
            if part_number.touches(symbol) {
                sum += part_number.value;
                continue;
            }
        }
    }
    sum
}

fn solve2(input: &Vec<&str>) -> u32 {
    let mut row_index = 0;

    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    for line in input {
        let mut new_part_numbers = get_part_numbers(line, row_index);
        part_numbers.append(&mut new_part_numbers);
        let mut new_symbols = get_symbols(line, row_index);
        symbols.append(&mut new_symbols);
        row_index += 1;
    }

    let gears: Vec<Symbol> = symbols.into_iter().filter(|s| s.value == '*').collect();

    let mut sum = 0;
    for gear in gears {
        let parts = gear.touches(&part_numbers);
        if parts.len() == 2 {
            sum += parts[0].value * parts[1].value
        }
    }
    sum
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PartNumber {
    value: u32,
    row: u32,
    start_column: u32,
    end_column: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Symbol {
    value: char,
    row: u32,
    column: u32,
}

impl PartNumber {
    fn new(n: u32, row: u32, start_column: u32) -> Self {
        let digits = count_decimal_digits(n);
        PartNumber {
            value: n,
            row,
            start_column,
            end_column: start_column + digits - 1,
        }
    }
    fn touches(self: &Self, symbol: &Symbol) -> bool {
        if symbol.row > self.row + 1 || (self.row > 0 && symbol.row < self.row - 1) {
            return false;
        }
        if symbol.column > self.end_column + 1
            || (self.start_column > 0 && symbol.column < self.start_column - 1)
        {
            return false;
        }
        return true;
    }
}

impl Symbol {
    fn touches(self: &Self, part_numbers: &Vec<PartNumber>) -> Vec<PartNumber> {
        let mut touched: Vec<PartNumber> = Vec::new();
        for part_number in part_numbers {
            if part_number.touches(self) {
                touched.push(part_number.to_owned());
            }
        }
        touched
    }
}

fn count_decimal_digits(n: u32) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

fn get_part_numbers(input_row: &str, row: u32) -> Vec<PartNumber> {
    let mut numbers: Vec<PartNumber> = Vec::new();
    let mut temp = String::new();
    let mut column = 0;
    for c in input_row.chars() {
        if c.is_digit(10) {
            temp.push(c);
        } else {
            if !temp.is_empty() {
                let number: u32 = temp.parse().unwrap();
                numbers.push(PartNumber::new(
                    number,
                    row,
                    column - count_decimal_digits(number),
                ));
                temp.clear();
            }
        }
        column += 1;
    }

    if !temp.is_empty() {
        let number: u32 = temp.parse().unwrap();
        numbers.push(PartNumber::new(
            number,
            row,
            column - count_decimal_digits(number),
        ));
        temp.clear();
    }
    numbers
}

fn get_symbols(input_row: &str, row: u32) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut column = 0;
    for c in input_row.chars() {
        if !c.is_digit(10) && c != '.' {
            symbols.push(Symbol {
                value: c,
                row,
                column,
            })
        }
        column += 1;
    }

    symbols
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_count_decimal_digits_correctly() {
        assert_eq!(count_decimal_digits(10), 2);
        assert_eq!(count_decimal_digits(999), 3);
        assert_eq!(count_decimal_digits(327), 3);
        assert_eq!(count_decimal_digits(104113), 6);
    }

    #[test]
    fn symbol_touches_part_number_in_row_below() {
        let row = 10;
        let column = 20;
        let symbol = Symbol {
            value: '$',
            row,
            column,
        };
        let number = 100;

        let part = PartNumber::new(number, row + 1, symbol.column - 3);
        assert!(part.touches(&symbol));

        let part = PartNumber::new(number, row + 1, symbol.column - 2);
        assert!(part.touches(&symbol));

        let part = PartNumber::new(number, row + 1, symbol.column - 1);
        assert!(part.touches(&symbol));

        let part = PartNumber::new(number, row + 1, symbol.column);
        assert!(part.touches(&symbol));

        let part = PartNumber::new(number, row + 1, symbol.column + 1);
        assert!(part.touches(&symbol));
    }

    #[test]
    fn symbol_touches_part_number_in_row_above() {
        let row = 10;
        let column = 20;
        let symbol = Symbol {
            value: '$',
            row,
            column,
        };
        let number = 100;

        let part = PartNumber::new(number, row - 1, symbol.column - 3);
        assert!(part.touches(&symbol));

        let part = PartNumber::new(number, row - 1, symbol.column - 2);
        assert!(part.touches(&symbol));

        let part = PartNumber::new(number, row - 1, symbol.column - 1);
        assert!(part.touches(&symbol));

        let part = PartNumber::new(number, row - 1, symbol.column);
        assert!(part.touches(&symbol));

        let part = PartNumber::new(number, row - 1, symbol.column + 1);
        assert!(part.touches(&symbol));
    }

    #[test]
    fn symbol_touches_part_number_in_same_row() {
        let row = 10;
        let column = 20;
        let symbol = Symbol {
            value: '$',
            row,
            column,
        };
        let number = 100;

        let part = PartNumber::new(number, row, symbol.column - 3);
        assert!(part.touches(&symbol));

        let part = PartNumber::new(number, row, symbol.column + 1);
        assert!(part.touches(&symbol));
    }

    #[test]
    fn symbol_does_not_touch_part_number_in_same_row() {
        let row = 10;
        let column = 20;
        let symbol = Symbol {
            value: '$',
            row,
            column,
        };
        let number = 100;

        let part = PartNumber::new(number, row, symbol.column - 4);
        assert!(!part.touches(&symbol));

        let part = PartNumber::new(number, row, symbol.column + 2);
        assert!(!part.touches(&symbol));
    }

    #[test]
    fn symbol_does_not_touch_part_number_in_row_below() {
        let row = 10;
        let column = 20;
        let symbol = Symbol {
            value: '$',
            row,
            column,
        };
        let number = 100;

        let part = PartNumber::new(number, row + 1, symbol.column - 4);
        assert!(!part.touches(&symbol));

        let part = PartNumber::new(number, row + 1, symbol.column + 2);
        assert!(!part.touches(&symbol));
    }

    #[test]
    fn symbol_does_not_touch_part_number_in_row_above() {
        let row = 10;
        let column = 20;
        let symbol = Symbol {
            value: '$',
            row,
            column,
        };
        let number = 100;

        let part = PartNumber::new(number, row - 1, symbol.column - 4);
        assert!(!part.touches(&symbol));

        let part = PartNumber::new(number, row - 1, symbol.column + 2);
        assert!(!part.touches(&symbol));
    }

    #[test]
    fn parse_part_numbers_from_input_row() {
        let input_row = "ab..c123de..f456";

        let numbers = get_part_numbers(input_row, 1);

        assert_eq!(
            numbers,
            vec![PartNumber::new(123, 1, 5), PartNumber::new(456, 1, 13)]
        )
    }
    #[test]
    fn parse_symbols_from_input_row() {
        let input_row = "ab..c123de..f456";

        let symbols = get_symbols(input_row, 1);

        assert_eq!(
            symbols,
            vec![
                Symbol {
                    value: 'a',
                    row: 1,
                    column: 0
                },
                Symbol {
                    value: 'b',
                    row: 1,
                    column: 1
                },
                Symbol {
                    value: 'c',
                    row: 1,
                    column: 4
                },
                Symbol {
                    value: 'd',
                    row: 1,
                    column: 8
                },
                Symbol {
                    value: 'e',
                    row: 1,
                    column: 9
                },
                Symbol {
                    value: 'f',
                    row: 1,
                    column: 12
                },
            ]
        )
    }

    #[test]
    fn can_calculate_sum_for_sample_input_part1() {
        let input: Vec<&str> = include_str!("../data/sample_input.txt").lines().collect();
        let sum = solve1(&input);
        assert_eq!(sum, 4361);
    }

    #[test]
    fn can_calculate_sum_for_sample_input_part2() {
        let input: Vec<&str> = include_str!("../data/sample_input.txt").lines().collect();
        let sum = solve2(&input);
        assert_eq!(sum, 467835);
    }
}
