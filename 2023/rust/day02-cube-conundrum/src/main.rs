// Task:
// Part 1:
// - Parse a file line by line
// - game is possible if red <= 12, green <= 13, green <= 14
// - For every line, one line per game
//      - split game into turns (separated by ";")
//      - check if game is possible given every turn
//          - if game is possible, add game id to sum, then proceed with next line
//          - if not, go to next line
// Part 2:
// - ...

const RED_LIMIT: u32 = 12;
const GREEN_LIMIT: u32 = 13;
const BLUE_LIMIT: u32 = 14;

#[derive(Debug, PartialEq, Eq)]
struct Turn {
    red: u32,
    blue: u32,
    green: u32,
}

impl Turn {
    fn is_possible(self: &Self) -> bool {
        self.red <= RED_LIMIT && self.green <= GREEN_LIMIT && self.blue <= BLUE_LIMIT
    }

    fn from(text: &str) -> Self {
        let mut turn = Turn {
            red: 0,
            blue: 0,
            green: 0,
        };
        let terms: Vec<&str> = text.split(',').map(|s| s.trim()).collect();

        for term in terms {
            if term.contains("red") {
                let red_value: Vec<&str> = term.split("red").map(|s| s.trim()).collect();
                let value = red_value[0].parse::<u32>();
                match value {
                    Ok(result) => turn.red = result,
                    Err(_) => panic!("something went wrong while parsing red"),
                }
            } else if term.contains("green") {
                let green_value: Vec<&str> = term.split("green").map(|s| s.trim()).collect();
                let value = green_value[0].parse::<u32>();
                match value {
                    Ok(result) => turn.green = result,
                    Err(_) => panic!("something went wrong while parsing green"),
                }
            } else if term.contains("blue") {
                let blue_value: Vec<&str> = term.split("blue").map(|s| s.trim()).collect();
                let value = blue_value[0].parse::<u32>();
                match value {
                    Ok(result) => turn.blue = result,
                    Err(_) => panic!("something went wrong while parsing blue"),
                }
            }
        }

        turn
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    turns: Vec<Turn>,
    max_red: u32,
    max_blue: u32,
    max_green: u32,
    power: u32,
}

impl Game {
    fn new(id: u32, turns: Vec<Turn>) -> Self {
        let max_red = get_max_red(&turns);
        let max_green = get_max_green(&turns);
        let max_blue = get_max_blue(&turns);
        Game {
            id,
            turns,
            max_red,
            max_blue,
            max_green,
            power: max_red * max_green * max_blue,
        }
    }

    fn is_possible(self: &Self) -> bool {
        for turn in &self.turns {
            if !turn.is_possible() {
                return false;
            }
        }
        true
    }

    fn from(text: &str) -> Self {
        let terms: Vec<&str> = text.split(':').map(|s| s.trim()).collect();

        // parse game id
        let id_terms: Vec<&str> = terms[0].split("Game").map(|s| s.trim()).collect();
        let game_id = id_terms[1].parse::<u32>();
        let id = match game_id {
            Ok(result) => result,
            Err(_) => panic!("Can't parse id from line"),
        };
        // parse turns
        let mut turns: Vec<Turn> = vec![];
        let turn_terms: Vec<&str> = terms[1].split(";").map(|s| s.trim()).collect();

        for turn_text in turn_terms {
            let turn = Turn::from(turn_text);
            turns.push(turn);
        }

        Game::new(id, turns)
    }
}

fn get_max_red(turns: &Vec<Turn>) -> u32 {
    let mut max = 0;
    for turn in turns {
        if turn.red > max {
            max = turn.red;
        }
    }

    max
}

fn get_max_green(turns: &Vec<Turn>) -> u32 {
    let mut max = 0;
    for turn in turns {
        if turn.green > max {
            max = turn.green;
        }
    }

    max
}

fn get_max_blue(turns: &Vec<Turn>) -> u32 {
    let mut max = 0;
    for turn in turns {
        if turn.blue > max {
            max = turn.blue;
        }
    }

    max
}

fn main() {
    let input: Vec<&str> = include_str!("../data/input.txt").lines().collect();

    let mut possible_sum = 0;
    let mut power_sum = 0;
    for line in input {
        let game = Game::from(line);
        if game.is_possible() {
            possible_sum += game.id;
        }
        power_sum += game.power;
    }

    println!("Sum of possible game ids: {}", possible_sum);
    println!("Sum of power of games: {}", power_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn possible_turns_are_recognized_as_possible() {
        let turn = Turn {
            red: 10,
            blue: 5,
            green: 13,
        };
        assert!(turn.is_possible());
    }

    #[test]
    fn impossible_turns_are_recognized_as_not_possible() {
        let turn = Turn {
            red: 10,
            blue: 16,
            green: 13,
        };
        assert!(!turn.is_possible());
    }

    #[test]
    fn game_with_only_possible_turns_is_recognized_as_possible() {
        let game = Game::new(
            1,
            vec![
                Turn {
                    red: 10,
                    blue: 5,
                    green: 13,
                },
                Turn {
                    red: 10,
                    blue: 11,
                    green: 13,
                },
                Turn {
                    red: 2,
                    blue: 5,
                    green: 0,
                },
            ],
        );

        assert!(game.is_possible());
    }

    #[test]
    fn game_with_at_least_one_impossible_turn_is_recognized_as_not_possible() {
        let game = Game::new(
            2,
            vec![
                Turn {
                    red: 10,
                    blue: 5,
                    green: 13,
                },
                Turn {
                    red: 10,
                    blue: 11,
                    green: 19,
                },
                Turn {
                    red: 2,
                    blue: 5,
                    green: 0,
                },
            ],
        );

        assert!(!game.is_possible());
    }

    #[test]
    fn can_parse_turns_from_text() {
        // case 1
        let turn = Turn::from("5 red, 3 green, 1 blue");
        assert_eq!(
            turn,
            Turn {
                red: 5,
                green: 3,
                blue: 1
            }
        );

        // case 2
        let turn = Turn::from("10 blue, 7 green, 1 red");
        assert_eq!(
            turn,
            Turn {
                red: 1,
                green: 7,
                blue: 10
            }
        );
    }

    #[test]
    fn can_parse_game_from_text() {
        let game = Game::from("Game 113: 10 blue, 7 green, 1 red; 5 red, 3 green, 1 blue; 10 blue");
        assert_eq!(
            game,
            Game::new(
                113,
                vec![
                    Turn {
                        red: 1,
                        green: 7,
                        blue: 10
                    },
                    Turn {
                        red: 5,
                        green: 3,
                        blue: 1
                    },
                    Turn {
                        red: 0,
                        green: 0,
                        blue: 10
                    }
                ]
            )
        );
    }

    #[test]
    fn calculate_power_of_game_correctly() {
        let game = Game::new(
            113,
            vec![
                Turn {
                    red: 1,
                    green: 7,
                    blue: 10,
                },
                Turn {
                    red: 5,
                    green: 3,
                    blue: 1,
                },
                Turn {
                    red: 0,
                    green: 0,
                    blue: 10,
                },
            ],
        );

        assert_eq!(game.max_red, 5);
        assert_eq!(game.max_green, 7);
        assert_eq!(game.max_blue, 10);
        assert_eq!(game.power, 10 * 7 * 5)
    }

    #[test]
    fn possible_sum_is_correct_on_sample_input() {
        let input: Vec<&str> = include_str!("../data/sample_input.txt").lines().collect();

        let mut possible_sum = 0;
        for line in input {
            let game = Game::from(line);
            if game.is_possible() {
                possible_sum += game.id;
            }
        }

        assert_eq!(possible_sum, 8);
    }

    #[test]
    fn power_sum_is_correct_on_sample_input() {
        let input: Vec<&str> = include_str!("../data/sample_input.txt").lines().collect();

        let mut power_sum = 0;
        for line in input {
            let game = Game::from(line);
            power_sum += game.power;
        }

        assert_eq!(power_sum, 2286);
    }
}
