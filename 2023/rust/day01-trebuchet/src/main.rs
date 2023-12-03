// Task:
// - Parse a file line by line
// - For every line, combine the first and last digit
// - Build the sum

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("data/input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        let number = parse_number_from_string(&line);
        sum += number;
    }

    println!("Total: {}", sum);
    Ok(())
}

fn parse_number_from_string(s: &str) -> u32 {
    let first_number = s.chars().filter(|c| c.is_numeric()).next();
    let first_number = match first_number {
        Some(num) => num.to_digit(10),
        None => panic!("No number in string"),
    };
    let first_number = match first_number {
        Some(num) => num,
        None => panic!("Conversion to number failed for first_number"),
    };

    let last_number = s.chars().rev().filter(|c| c.is_numeric()).next();
    let last_number = match last_number {
        Some(num) => num.to_digit(10),
        None => panic!("No number in string"),
    };
    let last_number = match last_number {
        Some(num) => num,
        None => panic!("Conversion to number failed for last_number"),
    };

    10 * first_number + last_number
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_parse_number_from_string() {
        let number = parse_number_from_string("1aa2");
        assert_eq!(number, 12);
        let number = parse_number_from_string("bc3aa4de5bad");
        assert_eq!(number, 35);
        let number = parse_number_from_string("aa123bbd13152dad313doo");
        assert_eq!(number, 13);
    }
}
