// Task:
// Part 1:
// - Determine the amount of possible options to win a race
//   - Notation:
//     - Time: x y z
//     - Distance: X Y Z
//     - example: x=4 -> 4ms X=10 -> 10mm
//   - every full ms of pressing the button at the start, increases the speed by 1 mm / ms
//   - initial speed 0
// - Multiply the option count for every race
// Part 2:
// - Same as before except it's only 1 race
// - Example:
//   - Time: 10 9 8
//   - Distance 5 4 3
//   - is meant to be 1 race with 1098 record time over a distance of 543
fn main() {
    let input: Vec<&str> = include_str!("../data/input.txt").lines().collect();
    let result1 = solve1(&input);
    println!("Result Part 1: {}", result1);

    let result2 = solve2(&input);
    println!("Result Part 2: {}", result2);
}

fn solve1(input: &[&str]) -> u64 {
    let sheet = RaceSheet::from(input, 1);

    sheet.result_of_part1()
}

fn solve2(input: &[&str]) -> u64 {
    let sheet = RaceSheet::from(input, 2);

    sheet.result_of_part2()
}

#[derive(Debug)]
struct Car {
    current_speed: u64,
    acceleration: u64,
}

impl Car {
    fn new() -> Self {
        Car {
            current_speed: 0,
            acceleration: 1,
        }
    }

    fn accelerate(&mut self, time: u64) -> u64 {
        self.current_speed += self.acceleration * time;

        self.current_speed
    }

    fn distance(&self, time: u64) -> u64 {
        self.current_speed * time
    }
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn winning_options(&self) -> Vec<u64> {
        let mut car = Car::new();
        let mut options = vec![];
        for charge_time in 1..self.time {
            car.accelerate(1);
            if car.distance(self.time - charge_time) > self.distance {
                options.push(charge_time);
            }
        }
        options
    }
}

#[derive(Debug)]
struct RaceSheet {
    races: Vec<Race>,
}

impl RaceSheet {
    fn from(input: &[&str], part: u64) -> RaceSheet {
        if part == 1 {
            let time_terms: Vec<&str> = input[0].split(':').map(|s| s.trim()).collect();
            let times: Vec<u64> = time_terms[1]
                .split(' ')
                .filter_map(|s| s.parse::<u64>().ok())
                .collect();

            let distance_terms: Vec<&str> = input[1].split(':').map(|s| s.trim()).collect();
            let distances: Vec<u64> = distance_terms[1]
                .split(' ')
                .filter_map(|s| s.parse::<u64>().ok())
                .collect();

            println!("Times: {:?}", times);
            println!("Distance: {:?}", distances);

            let mut races = vec![];
            for i in 0..times.len() {
                races.push(Race {
                    time: times[i],
                    distance: distances[i],
                })
            }

            RaceSheet { races }
        } else if part == 2 {
            let time_terms: Vec<&str> = input[0].split(':').map(|s| s.trim()).collect();
            let time = time_terms[1].replace(' ', "").parse::<u64>();
            let time = match time {
                Ok(value) => value,
                Err(_) => panic!(""),
            };

            let distance_terms: Vec<&str> = input[1].split(':').map(|s| s.trim()).collect();
            let distance = distance_terms[1].replace(' ', "").parse::<u64>();
            let distance = match distance {
                Ok(value) => value,
                Err(_) => panic!(""),
            };

            println!("Time: {:?}", time);
            println!("Distance: {:?}", distance);

            let races = vec![Race { time, distance }];

            RaceSheet { races }
        } else {
            todo!()
        }
    }

    fn result_of_part1(&self) -> u64 {
        let mut result = 1;
        for race in &self.races {
            let options = race.winning_options();
            println!("Options for {:?}: {}", race, options.len());
            result *= options.len();
        }

        result as u64
    }

    fn result_of_part2(&self) -> u64 {
        let mut result = 1;
        for race in &self.races {
            let options = race.winning_options();
            println!("Options for {:?}: {}", race, options.len());
            result *= options.len();
        }

        result as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1_with_sample_input() {
        let input: Vec<&str> = include_str!("../data/sample_input.txt").lines().collect();
        let result = solve1(&input);

        assert_eq!(result, 288);
    }

    #[test]
    fn can_solve_part2_with_sample_input() {
        let input: Vec<&str> = include_str!("../data/sample_input.txt").lines().collect();
        let result = solve2(&input);

        assert_eq!(result, 71503);
    }
}
