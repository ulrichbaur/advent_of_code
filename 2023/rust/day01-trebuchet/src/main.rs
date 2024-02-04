// Task:
// Part 1:
// - Parse a file line by line
// - For every line
//      - combine the first and last digit
// - Build the sum
// Part 2:
// - Parse a file line by line
// - For every line
//      - combine the first and last digit
//      - counting words of digits (e.g. one, two, three, ...)
// - Build the sum

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("data/input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        let number = parse_number_from_string(&line);
        // println!("{}: {}", line, number);
        sum += number;
    }

    println!("Total: {}", sum);
    Ok(())
}

fn parse_number_from_string(s: &str) -> u32 {
    let first_number = find_first_digit_in_string(s);

    let last_number = find_last_digit_in_string(s);

    10 * first_number + last_number
}

const DIGITS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];
fn find_first_digit_in_string(s: &str) -> u32 {
    let mut map: HashMap<String, u32> = Default::default();
    for digit in DIGITS.iter() {
        let index = s.find(digit);
        match index {
            Some(num) => map.insert(digit.to_string(), num.try_into().unwrap()),
            None => continue,
        };
    }

    let (key, _) = map.iter().min_by_key(|&(_, v)| v).unwrap();

    match_key_to_digit(key)
}

fn find_last_digit_in_string(s: &str) -> u32 {
    let mut map: HashMap<String, u32> = Default::default();
    for digit in DIGITS.iter() {
        let index = s.rfind(digit);
        match index {
            Some(num) => map.insert(digit.to_string(), num.try_into().unwrap()),
            None => continue,
        };
    }

    let (key, _) = map.iter().max_by_key(|&(_, v)| v).unwrap();

    match_key_to_digit(key)
}

fn match_key_to_digit(key: &str) -> u32 {
    match key {
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("Something else"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_parse_raw_digits_from_string() {
        let first_digit = find_first_digit_in_string("123");
        let last_digit = find_last_digit_in_string("123");
        assert_eq!(first_digit, 1);
        assert_eq!(last_digit, 3);

        let first_digit = find_first_digit_in_string("567");
        let last_digit = find_last_digit_in_string("567");
        assert_eq!(first_digit, 5);
        assert_eq!(last_digit, 7);
    }

    #[test]
    fn can_parse_digits_as_words_from_string() {
        let first_digit = find_first_digit_in_string("onetwothree");
        let last_digit = find_last_digit_in_string("onetwothree");
        assert_eq!(first_digit, 1);
        assert_eq!(last_digit, 3);

        let first_digit = find_first_digit_in_string("fivesixseven");
        let last_digit = find_last_digit_in_string("fivesixseven");
        assert_eq!(first_digit, 5);
        assert_eq!(last_digit, 7);
    }
    #[test]
    fn can_parse_mix_of_raw_digits_and_digits_as_words_from_string() {
        let first_digit = find_first_digit_in_string("onetwo3");
        let last_digit = find_last_digit_in_string("onetwo3");
        assert_eq!(first_digit, 1);
        assert_eq!(last_digit, 3);

        let first_digit = find_first_digit_in_string("5sixseven");
        let last_digit = find_last_digit_in_string("5sixseven");
        assert_eq!(first_digit, 5);
        assert_eq!(last_digit, 7);
    }
    #[test]
    fn can_parse_mix_of_raw_digits_and_digits_as_words_with_added_noise_from_string() {
        let first_digit = find_first_digit_in_string("sdlonejakdstwo3sadad");
        let last_digit = find_last_digit_in_string("sdlonejakdstwo3sadad");
        assert_eq!(first_digit, 1);
        assert_eq!(last_digit, 3);

        let first_digit = find_first_digit_in_string("bsaf5sixahasdasevenfadf");
        let last_digit = find_last_digit_in_string("bsaf5sixahasdasevenfadf");
        assert_eq!(first_digit, 5);
        assert_eq!(last_digit, 7);
    }

    #[test]
    fn can_parse_number_from_mixed_string() {
        let number = parse_number_from_string("sdlonejakdstwo3sadad");
        assert_eq!(number, 13);

        let number = parse_number_from_string("bsaf5sixahasdasevenfadf");
        assert_eq!(number, 57);
    }
}
