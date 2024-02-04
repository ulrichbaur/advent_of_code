// Task:
// Part 1:
// - Walk seeds through maps
// - Maps map a source range to a destination range
//   - dest-range src-range range-length
//   - example, 10 20 5 maps 20..24 to 10..14
//   - otherwise, src = dest
// - Find lowest location number
// Part 2:
// - Same as before, but
// - Seeds are not listed, but also ranges
//   - pairs of numbers, first number is start of range, second value is the range
//   - example, 79 14 corresponds to seeds 79..93

//------------------------------------------------------------------//
//                              Sample                              //
//------------------------------------------------------------------//
//
// seeds: 79 14 55 13
//
// seed-to-soil map:
// 50 98 2
// 52 50 48
//
// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15
//
// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4
//
// water-to-light map:
// 88 18 7
// 18 25 70
//
// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13
//
// temperature-to-humidity map:
// 0 69 1
// 1 0 69
//
// humidity-to-location map:
// 60 56 37
// 56 93 4
//
// Sample Solution Part 1:
// - Seed 79 -> Soil 81 -> Fertilizer 81 -> Water 81 -> Light 74 -> Temperature 78 -> Humidity 78 -> Location 82
// - Seed 14 -> Soil 14 -> Fertilizer 53 -> Water 49 -> Light 42 -> Temperature 42 -> Humidity 43 -> Location 43
// - Seed 55 -> Soil 57 -> Fertilizer 57 -> Water 53 -> Light 46 -> Temperature 82 -> Humidity 82 -> Location 86
// - Seed 13 -> Soil 13 -> Fertilizer 52 -> Water 41 -> Light 34 -> Temperature 34 -> Humidity 35 -> Location 35
// - Lowest Location: 35
//
// Sample Solution Part 2:
//
// - Seed 79 -> Soil 81 -> Fertilizer 81 -> Water 81 -> Light 74 -> Temperature 78 -> Humidity 78 -> Location 82
// - Seed 80 -> Soil 82 -> Fertilizer 82 -> Water 82 -> Light 75 -> Temperature 79 -> Humidity 79 -> Location 83
// - Seed 81 -> Soil 83 -> Fertilizer 83 -> Water 83 -> Light 76 -> Temperature 80 -> Humidity 80 -> Location 84
// - Seed 82 -> Soil 84 -> Fertilizer 84 -> Water 84 -> Light 77 -> Temperature 45 -> Humidity 46 -> Location 46
// - Seed 83 -> Soil 85 -> Fertilizer 85 -> Water 85 -> Light 78 -> Temperature 46 -> Humidity 47 -> Location 47
// - Seed 84 -> Soil 86 -> Fertilizer 86 -> Water 86 -> Light 79 -> Temperature 47 -> Humidity 48 -> Location 48
// - Seed 85 -> Soil 87 -> Fertilizer 87 -> Water 87 -> Light 80 -> Temperature 48 -> Humidity 49 -> Location 49
// - Seed 86 -> Soil 88 -> Fertilizer 88 -> Water 88 -> Light 81 -> Temperature 49 -> Humidity 50 -> Location 50
// - Seed 87 -> Soil 89 -> Fertilizer 89 -> Water 89 -> Light 82 -> Temperature 50 -> Humidity 51 -> Location 51
// - Seed 88 -> Soil 90 -> Fertilizer 90 -> Water 90 -> Light 83 -> Temperature 51 -> Humidity 52 -> Location 52
// - Seed 89 -> Soil 91 -> Fertilizer 91 -> Water 91 -> Light 84 -> Temperature 52 -> Humidity 53 -> Location 53
// - Seed 90 -> Soil 92 -> Fertilizer 92 -> Water 92 -> Light 85 -> Temperature 53 -> Humidity 54 -> Location 54
// - Seed 91 -> Soil 93 -> Fertilizer 93 -> Water 93 -> Light 86 -> Temperature 54 -> Humidity 55 -> Location 55
// - Seed 92 -> Soil 94 -> Fertilizer 94 -> Water 94 -> Light 87 -> Temperature 55 -> Humidity 56 -> Location 60
// - Seed 55 -> Soil 57 -> Fertilizer 57 -> Water 53 -> Light 46 -> Temperature 82 -> Humidity 82 -> Location 86
// - Seed 56 -> Soil 58 -> Fertilizer 58 -> Water 54 -> Light 47 -> Temperature 83 -> Humidity 83 -> Location 87
// - Seed 57 -> Soil 59 -> Fertilizer 59 -> Water 55 -> Light 48 -> Temperature 84 -> Humidity 84 -> Location 88
// - Seed 58 -> Soil 60 -> Fertilizer 60 -> Water 56 -> Light 49 -> Temperature 85 -> Humidity 85 -> Location 89
// - Seed 59 -> Soil 61 -> Fertilizer 61 -> Water 61 -> Light 54 -> Temperature 90 -> Humidity 90 -> Location 94
// - Seed 60 -> Soil 62 -> Fertilizer 62 -> Water 62 -> Light 55 -> Temperature 91 -> Humidity 91 -> Location 95
// - Seed 61 -> Soil 63 -> Fertilizer 63 -> Water 63 -> Light 56 -> Temperature 92 -> Humidity 92 -> Location 96
// - Seed 62 -> Soil 64 -> Fertilizer 64 -> Water 64 -> Light 57 -> Temperature 93 -> Humidity 93 -> Location 56
// - Seed 63 -> Soil 65 -> Fertilizer 65 -> Water 65 -> Light 58 -> Temperature 94 -> Humidity 94 -> Location 57
// - Seed 64 -> Soil 66 -> Fertilizer 66 -> Water 66 -> Light 59 -> Temperature 95 -> Humidity 95 -> Location 58
// - Seed 65 -> Soil 67 -> Fertilizer 67 -> Water 67 -> Light 60 -> Temperature 96 -> Humidity 96 -> Location 59
// - Seed 66 -> Soil 68 -> Fertilizer 68 -> Water 68 -> Light 61 -> Temperature 97 -> Humidity 97 -> Location 97
// - Seed 67 -> Soil 69 -> Fertilizer 69 -> Water 69 -> Light 62 -> Temperature 98 -> Humidity 98 -> Location 98
// - Lowest Location: 46

fn main() {
    let input = include_str!("../data/input.txt");

    let result1 = solve1(input);
    println!("Result Part 1: {}", result1);

    println!();

    let result2 = solve2(input);
    println!("Result Part 2: {}", result2);
}

fn solve1(input: &str) -> u64 {
    let terms: Vec<&str> = input.split("\n\n").collect();

    if terms.len() != 8 {
        panic!(
            "Not enough terms in input file; expected 8, got {}",
            terms.len()
        )
    }

    let seed_location = SeedLocation::from(&terms, 1);

    let seed_count = seed_location.seeds.len();
    println!("Mapping {} seeds to locations", seed_count);

    let mut min_location = 2_u64.pow(64) - 1;
    for seed in &seed_location.seeds {
        let location = seed_location.map_seed_to_location(*seed);
        if location < min_location {
            println!("New min location: {}", location);
            min_location = location;
        }
    }

    min_location
}

fn solve2(input: &str) -> u64 {
    let terms: Vec<&str> = input.split("\n\n").collect();

    if terms.len() != 8 {
        panic!(
            "Not enough terms in input file; expected 5, got {}",
            terms.len()
        )
    }

    let seed_location = SeedLocation::from(&terms, 2);

    let seed_count = seed_location.seeds.len();
    println!("Mapping {} seeds to locations", seed_count);

    let mut min_location = 2_u64.pow(64) - 1;
    let mut seed_index: u64 = 0;
    for seed in &seed_location.seeds {
        let location = seed_location.map_seed_to_location(*seed);
        if location < min_location {
            println!("New min location: {}", location);
            min_location = location;
        }
        seed_index += 1;
        if seed_index % 5_000_000 == 0 {
            let ratio = seed_index as f64 / seed_count as f64;
            println!(
                "Finished {:.2}% ({} of {})",
                ratio * 100.0,
                seed_index,
                seed_count
            )
        }
    }

    min_location
}

fn parse_seeds(text: &str, part: u8) -> Vec<u64> {
    let terms: Vec<&str> = text.split(':').map(|s| s.trim()).collect();

    let seeds = match part {
        1 => terms[1]
            .split(' ')
            .filter_map(|s| s.parse::<u64>().ok())
            .collect(),
        2 => {
            let numbers: Vec<u64> = terms[1]
                .split(' ')
                .filter_map(|s| s.parse::<u64>().ok())
                .collect();
            let mut seeds = vec![];
            for i in (0..numbers.len()).step_by(2) {
                for seed in numbers[i]..numbers[i] + numbers[i + 1] {
                    seeds.push(seed);
                }
            }
            seeds
        }
        _ => todo!(),
    };

    seeds
}

#[derive(Debug, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    src: Range,
    dest: Range,
}

impl Map {
    fn from(input: &str) -> Self {
        let numbers: Vec<u64> = input
            .split(' ')
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();
        Map {
            src: Range {
                start: numbers[1],
                end: numbers[1] + numbers[2] - 1,
            },
            dest: Range {
                start: numbers[0],
                end: numbers[0] + numbers[2] - 1,
            },
        }
    }

    fn is_input_in_src_range(&self, src: u64) -> bool {
        src >= self.src.start && src <= self.src.end
    }

    fn offset_of(&self, src: u64) -> u64 {
        if !self.is_input_in_src_range(src) {
            panic!("{} does not exit in {:?}", src, self);
        }
        src - self.src.start
    }
    fn map_src_to_dest(&self, src: u64) -> u64 {
        if !self.is_input_in_src_range(src) {
            return src;
        }
        let offset = self.offset_of(src);

        self.dest.start + offset
    }
}

fn parse_vector_of_maps_from_string_vector(input: &[&str]) -> Vec<Map> {
    input.iter().map(|l| Map::from(l)).collect()
}

#[derive(Debug)]
struct SeedLocation {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Map>,
    soil_to_fertilizer: Vec<Map>,
    fertilizer_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temperature: Vec<Map>,
    temperature_to_humidity: Vec<Map>,
    humidity_to_location: Vec<Map>,
}

impl SeedLocation {
    fn from(input: &[&str], part: u8) -> SeedLocation {
        let seeds = parse_seeds(input[0], part);

        let seed_to_soil_lines: Vec<&str> = input[1].lines().skip(1).collect();
        let seed_to_soil = parse_vector_of_maps_from_string_vector(&seed_to_soil_lines);

        let soil_to_fertilizer_lines: Vec<&str> = input[2].lines().skip(1).collect();
        let soil_to_fertilizer = parse_vector_of_maps_from_string_vector(&soil_to_fertilizer_lines);

        let fertilizer_to_water_lines: Vec<&str> = input[3].lines().skip(1).collect();
        let fertilizer_to_water =
            parse_vector_of_maps_from_string_vector(&fertilizer_to_water_lines);

        let water_to_light_lines: Vec<&str> = input[4].lines().skip(1).collect();
        let water_to_light = parse_vector_of_maps_from_string_vector(&water_to_light_lines);

        let light_to_temperature_lines: Vec<&str> = input[5].lines().skip(1).collect();
        let light_to_temperature =
            parse_vector_of_maps_from_string_vector(&light_to_temperature_lines);

        let temperature_to_humidity_lines: Vec<&str> = input[6].lines().skip(1).collect();
        let temperature_to_humidity =
            parse_vector_of_maps_from_string_vector(&temperature_to_humidity_lines);

        let humidity_to_location: Vec<&str> = input[7].lines().skip(1).collect();
        let humidity_to_location = parse_vector_of_maps_from_string_vector(&humidity_to_location);

        println!("Finished constructing SeedLocation");

        SeedLocation {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    fn map_seed_to_location(&self, seed: u64) -> u64 {
        print!("Seed {} -> ", seed);

        let mut soil = seed;
        for map in &self.seed_to_soil {
            if map.is_input_in_src_range(soil) {
                soil = map.map_src_to_dest(soil);
                break;
            }
        }
        print!("Soil {} -> ", soil);

        let mut fertilizer = soil;
        for map in &self.soil_to_fertilizer {
            if map.is_input_in_src_range(fertilizer) {
                fertilizer = map.map_src_to_dest(fertilizer);
                break;
            }
        }
        print!("Fertilizer {} -> ", fertilizer);

        let mut water = fertilizer;
        for map in &self.fertilizer_to_water {
            if map.is_input_in_src_range(water) {
                water = map.map_src_to_dest(water);
                break;
            }
        }
        print!("Water {} -> ", water);

        let mut light = water;
        for map in &self.water_to_light {
            if map.is_input_in_src_range(light) {
                light = map.map_src_to_dest(light);
                break;
            }
        }
        print!("Light {} -> ", light);

        let mut temperature = light;
        for map in &self.light_to_temperature {
            if map.is_input_in_src_range(temperature) {
                temperature = map.map_src_to_dest(temperature);
                break;
            }
        }
        print!("Temperature {} -> ", temperature);

        let mut humidity = temperature;
        for map in &self.temperature_to_humidity {
            if map.is_input_in_src_range(humidity) {
                humidity = map.map_src_to_dest(humidity);
                break;
            }
        }
        print!("Humidity {} -> ", humidity);

        let mut location = humidity;
        for map in &self.humidity_to_location {
            if map.is_input_in_src_range(location) {
                location = map.map_src_to_dest(location);
                break;
            }
        }
        println!("Location {}", location);

        location
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_seeds_from_string_for_part1() {
        let input = "seeds: 63 123 6243 123";
        let seeds = parse_seeds(input, 1);

        assert_eq!(seeds, vec![63, 123, 6243, 123]);

        let input = "seeds: 152 643 742 302 910 120";
        let seeds = parse_seeds(input, 1);

        assert_eq!(seeds, vec![152, 643, 742, 302, 910, 120]);
    }

    #[test]
    fn can_parse_seeds_from_string_for_part2() {
        let input = "seeds: 1 3 10 2";
        let seeds = parse_seeds(input, 2);

        assert_eq!(seeds, vec![1, 2, 3, 10, 11]);
    }

    #[test]
    fn can_parse_map_from_string() {
        let input = "63 123 25";
        let map = Map::from(input);

        assert_eq!(
            map,
            Map {
                src: Range {
                    start: 123,
                    end: 123 + 25 - 1
                },
                dest: Range {
                    start: 63,
                    end: 63 + 25 - 1
                }
            }
        );
    }

    #[test]
    fn can_map_input_to_output() {
        let input = "63 123 25";
        let map = Map::from(input);

        assert_eq!(map.map_src_to_dest(1), 1);
        assert_eq!(map.map_src_to_dest(10), 10);
        assert_eq!(map.map_src_to_dest(67), 67);

        assert_eq!(map.map_src_to_dest(125), 65);
        assert_eq!(map.map_src_to_dest(133), 73);
    }

    #[test]
    fn can_parse_vector_of_maps_from_vector_of_strings() {
        let input = vec!["54 11 17", "98 63 23", "151 120 35"];

        let map_vector = parse_vector_of_maps_from_string_vector(&input);

        assert_eq!(
            map_vector,
            vec![
                Map {
                    src: Range {
                        start: 11,
                        end: 11 + 17 - 1
                    },
                    dest: Range {
                        start: 54,
                        end: 54 + 17 - 1
                    }
                },
                Map {
                    src: Range {
                        start: 63,
                        end: 63 + 23 - 1
                    },
                    dest: Range {
                        start: 98,
                        end: 98 + 23 - 1
                    }
                },
                Map {
                    src: Range {
                        start: 120,
                        end: 120 + 35 - 1
                    },
                    dest: Range {
                        start: 151,
                        end: 151 + 35 - 1
                    }
                }
            ]
        )
    }

    #[test]
    fn can_solve_sample_input_for_part_1() {
        let input = include_str!("../data/sample_input.txt");
        let result = solve1(input);

        assert_eq!(result, 35);
    }

    #[test]
    fn can_solve_input_for_part_1() {
        let input = include_str!("../data/input.txt");
        let result = solve1(input);

        assert_eq!(result, 199602917);
    }

    #[test]
    fn can_solve_sample_input_for_part_2() {
        let input = include_str!("../data/sample_input.txt");
        let result = solve2(input);

        assert_eq!(result, 46);
    }

    #[test]
    #[ignore] // because it takes ~2 min
    fn can_solve_input_for_part_2() {
        let input = include_str!("../data/input.txt");
        let result = solve2(input);

        assert_eq!(result, 2254686);
    }
}
