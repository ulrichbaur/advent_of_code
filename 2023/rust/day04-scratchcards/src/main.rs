// Task:
// Part 1:
// - Parse winning numbers and numbers you have
// - One point for one match, each match after the first doubles the value
// - Sum up points
// Part 2:
// - For every match you win of the following cards
//   - i.e. 3 matches win a copy of the next 3 cards
//   - without going past the end of the table
// - Copies of cards are scored like normal cards
//   - also generate more copies
// - Sum up total amount of cards

//------------------------------------------------------------------//
//                              Sample                              //
//------------------------------------------------------------------//
// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
//
// Part 1 Sample Solution:
// - Card 1 has 4 winning numbers (48, 83, 17, and 86) -> 8 points
// - Card 2 has 2 winning numbers (32 and 61) -> 2 points
// - Card 3 has 2 winning numbers (1 and 21) -> 2 points
// - Card 4 has 1 winning number (84) -> 1 point
// - Card 5 has 0 winning numbers -> 0 points
// - Card 6 has 0 winning numbers -> 0 points
// - Total: 13 points
//
// Part 1 Sample Solution:
// - 1 * Card 1 has 4 winning numbers (48, 83, 17, and 86) -> 1 copy of 4 cards (2, 3, 4, 5)
// - 2 * Card 2 has 2 winning numbers (32 and 61) -> 2 copies of 2 cards (3, 4)
// - 4 * Card 3 has 2 winning numbers (1 and 21) -> 4 copies of 2 cards (4, 5)
// - 8 * Card 4 has 1 winning number (84) -> 8 of 1 card (5)
// - 14 * Card 5 has 0 winning numbers -> 0 copies
// - 1 * Card 6 has 0 winning numbers -> 0 copies
// - Total: 30 cards

fn main() {
    let input = include_str!("../data/input.txt").lines().collect();
    let result1 = solve1(&input);
    println!("Result Part 1: {}", result1);
    // let result2 = solve2(&input);
    // println!("Result Part 2: {}", result2);
}

fn solve1(input: &Vec<&str>) -> u32 {
    let mut sum = 0;
    for line in input {
        let card = Card::from(line);
        sum += card.points();
    }
    sum
}

fn solve2(input: &Vec<&str>) -> u32 {
    todo!()
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    drawn: Vec<u32>,
}

impl Card {
    fn points(self: &Self) -> u32 {
        let mut matches = 0;
        for drawn_card in &self.drawn {
            if self.winning.contains(drawn_card) {
                matches += 1;
            }
        }

        let points = match matches {
            0 => 0,
            _ => 2_u32.pow(matches - 1),
        };

        points
    }

    fn from(text: &str) -> Self {
        let terms: Vec<&str> = text.split(':').map(|s| s.trim()).collect();

        // parse id
        let id_terms: Vec<&str> = terms[0].split("Card").map(|s| s.trim()).collect();
        let card_id = match id_terms[1].parse::<u32>() {
            Ok(id) => id,
            Err(_) => panic!("Can't parse id from line"),
        };

        let card_terms: Vec<&str> = terms[1].split("|").map(|s| s.trim()).collect();
        // parse winning cards
        let winning_cards: Vec<u32> = card_terms[0]
            .split(" ")
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        // parse drawn cards
        let drawn_cards: Vec<u32> = card_terms[1]
            .split(" ")
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        return Card {
            id: card_id,
            winning: winning_cards,
            drawn: drawn_cards,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_card_from_string() {
        let input = "Card 02: 1 2 3 4 5 | 6 7 8 9 10";

        let card = Card::from(input);

        assert_eq!(
            card,
            Card {
                id: 2,
                winning: vec![1, 2, 3, 4, 5],
                drawn: vec![6, 7, 8, 9, 10]
            }
        )
    }

    #[test]
    fn can_calculate_points_for_card_with_0_winning() {
        let card = Card {
            id: 1,
            winning: vec![1, 2, 3, 4, 5],
            drawn: vec![6, 7, 8, 9, 10],
        };

        assert_eq!(card.points(), 0);
    }

    #[test]
    fn can_calculate_points_for_card_with_1_winning() {
        let card = Card {
            id: 1,
            winning: vec![1, 2, 3, 4, 5],
            drawn: vec![1, 6, 7, 8, 9],
        };
        assert_eq!(card.points(), 1);
    }

    #[test]
    fn can_calculate_points_for_card_with_2_winning() {
        let card = Card {
            id: 1,
            winning: vec![1, 2, 3, 4, 5],
            drawn: vec![1, 2, 6, 7, 8],
        };
        assert_eq!(card.points(), 2);
    }

    #[test]
    fn can_calculate_points_for_card_with_3_winning() {
        let card = Card {
            id: 1,
            winning: vec![1, 2, 3, 4, 5],
            drawn: vec![1, 2, 3, 6, 7],
        };
        assert_eq!(card.points(), 4);
    }

    #[test]
    fn can_calculate_points_for_card_with_4_winning() {
        let card = Card {
            id: 1,
            winning: vec![1, 2, 3, 4, 5],
            drawn: vec![1, 2, 3, 4, 6],
        };
        assert_eq!(card.points(), 8);
    }

    #[test]
    fn can_calculate_points_for_card_with_5_winning() {
        let card = Card {
            id: 1,
            winning: vec![1, 2, 3, 4, 5],
            drawn: vec![1, 2, 3, 4, 5],
        };
        assert_eq!(card.points(), 16);
    }

    #[test]
    fn can_solve_part1_for_sample_input() {
        let input: Vec<&str> = include_str!("../data/sample_input.txt").lines().collect();

        assert_eq!(solve1(&input), 13);
    }
}
