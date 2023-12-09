use std::fmt::Display;

// Task:
// Part 1:
// - Sort hands of cards
// - Notation: XXXXX YYY (X being the card, Y being the bid)
// - Available cards in order
//   - A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2
// - Hands can be 1 of multiple types
//   - five of a kind, four of a kind, full house, three of a kind, two pair, one pair, high card
// - Hands with the same rank are ranked by the comparing cards
//   - start with first card, higher card gets higher rank
//   - continue until fifth card
// - Multiply rank with bid to get winning of hand
// - Sum up the total winnings
// Part 2:
// - ...
fn main() {
    let input = include_str!("../data/input.txt").lines().collect();

    let result1 = solve1(&input);
    println!("Result Part 1: {}", result1);

    // let result2 = solve2(&input);
    // println!("Result Part 2: {}", result2);
}

fn solve1(input: &Vec<&str>) -> u32 {
    let mut hands: HandVec = HandVec(vec![]);
    for line in input {
        let hand = Hand::from(line);
        // println!("{:?}", hand.cards);
        hands.0.push(hand);
    }
    sort_vector_of_hands(&mut hands.0);
    println!("{}", hands);

    let mut sum = 0;

    for (i, hand) in hands.0.iter().enumerate() {
        sum += (i + 1) as u32 * hand.bid;
    }

    sum
}

fn solve2(input: &Vec<&str>) -> u32 {
    todo!()
}

const CARD_TYPES: [CardType; 13] = [
    CardType::Two,
    CardType::Three,
    CardType::Four,
    CardType::Five,
    CardType::Six,
    CardType::Seven,
    CardType::Eight,
    CardType::Nine,
    CardType::Ten,
    CardType::Jack,
    CardType::Queen,
    CardType::King,
    CardType::Ace,
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum CardType {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from(hand: &Vec<Card>) -> HandType {
        let mut occurrences: Vec<(CardType, u32)> = vec![];
        for card_type in CARD_TYPES {
            let count = hand.iter().filter(|&x| x.card_type == card_type).count();
            occurrences.push((card_type, count as u32));
        }
        occurrences.sort_by(|a, b| a.1.cmp(&b.1));
        let highest = &occurrences[12];
        let second_highest = &occurrences[11];
        // println!("Hand: {:?} {:?}", highest, second_highest);

        match highest.1 {
            1 => HandType::HighCard,
            2 => match second_highest.1 {
                1 => HandType::Pair,
                2 => HandType::TwoPair,
                _ => panic!(
                    "Failed to deduct hand type from {:?} {:?}",
                    highest, second_highest
                ),
            },
            3 => match second_highest.1 {
                1 => HandType::ThreeOfAKind,
                2 => HandType::FullHouse,
                _ => panic!(
                    "Failed to deduct hand type from {:?} {:?}",
                    highest, second_highest
                ),
            },
            4 => HandType::FourOfAKind,
            5 => HandType::FiveOfAKind,
            _ => panic!(
                "Failed to deduct hand type from {:?} {:?}",
                highest, second_highest
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    card_type: CardType,
}

impl Card {
    fn from(input_char: char) -> Card {
        let card_type = match input_char {
            'A' => CardType::Ace,
            'K' => CardType::King,
            'Q' => CardType::Queen,
            'J' => CardType::Jack,
            'T' => CardType::Ten,
            '9' => CardType::Nine,
            '8' => CardType::Eight,
            '7' => CardType::Seven,
            '6' => CardType::Six,
            '5' => CardType::Five,
            '4' => CardType::Four,
            '3' => CardType::Three,
            '2' => CardType::Two,
            _ => panic!("Unknown card type for {}", input_char),
        };

        Card { card_type }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: u32,
}

impl Hand {
    fn new(hand_input: &str, bid: u32) -> Hand {
        let mut cards = vec![];
        for card_input_char in hand_input.chars() {
            let card = Card::from(card_input_char);
            // println!("Parsed {:?}", card);
            cards.push(card);
        }
        let hand_type = HandType::from(&cards);
        // println!("{:?}", hand_type);
        Hand {
            cards,
            hand_type,
            bid,
        }
    }

    fn from(line: &str) -> Hand {
        let terms: Vec<&str> = line.split(" ").collect();
        let hand_string = terms[0];
        let bid = terms[1].parse::<u32>().unwrap();

        Hand::new(hand_string, bid)
    }
}

struct HandVec(Vec<Hand>);

impl Display for HandVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, hand) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, "\n")?;
            }
            write!(f, "{:?} ({:?})", hand.hand_type, hand.cards)?;
        }
        Ok(())
    }
}

fn sort_vector_of_hands(hands: &mut Vec<Hand>) {
    hands.sort_by(|a, b| {
        a.hand_type
            .cmp(&b.hand_type)
            .then_with(|| a.cards[0].cmp(&b.cards[0]))
            .then_with(|| a.cards[1].cmp(&b.cards[1]))
            .then_with(|| a.cards[2].cmp(&b.cards[2]))
            .then_with(|| a.cards[3].cmp(&b.cards[3]))
            .then_with(|| a.cards[4].cmp(&b.cards[4]))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1_for_sample_input() {
        let input = include_str!("../data/sample_input.txt").lines().collect();

        let result = solve1(&input);

        assert_eq!(result, 6440);
    }
}
