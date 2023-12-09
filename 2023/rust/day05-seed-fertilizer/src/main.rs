// Task:
// Part 1:
// - Walk seeds through maps
// - Maps map src-to-dest
//   - dest-range src-range range-length
//   - example, 10 20 5 maps 20..24 to 10..14
//   - otherwise, src = dest
// - Find lowest location number
// Part 2:
// - ...

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
// Part 1 Sample Solution:
// - seed 79, soil
// - Solution: ...
//
// Part 2 Sample Solution:
// - ...
// - Solution: ...

fn main() {
    let input = include_str!("../data/input.txt");
    let result1 = solve1(input);
    println!("Result Part 1: {}", result1);

    // let result2 = solve2(input);
    // println!("Result Part 2: {}", result2);
}

fn solve1(input: &str) -> u64 {
    let terms: Vec<&str> = input.split("\n\n").collect();

    if terms.len() != 8 {
        panic!(
            "Not enough terms in input file; expected 5, got {}",
            terms.len()
        )
    }

    let seed_location = SeedLocation::from(&terms);

    let mut locations = vec![];
    for seed in &seed_location.seeds {
        locations.push(seed_location.map_seed_to_location(*seed));
    }

    let min_value = locations.iter().min();

    let min = match min_value {
        Some(value) => value,
        None => panic!("Could not find minimum in {:?}", seed_location.seeds),
    };

    *min
}

fn solve2(input: &str) -> u64 {
    todo!()
}

fn parse_seeds(text: &str) -> Vec<u64> {
    let terms: Vec<&str> = text.split(':').map(|s| s.trim()).collect();
    terms[1]
        .split(" ")
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
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
            .split(" ")
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

    fn is_input_in_src_range(self: &Self, src: u64) -> bool {
        src >= self.src.start && src <= self.src.end
    }

    fn offset_of(self: &Self, src: u64) -> u64 {
        if !self.is_input_in_src_range(src) {
            panic!("{} does not exit in {:?}", src, self);
        }
        src - self.src.start
    }
    fn map_src_to_dest(self: &Self, src: u64) -> u64 {
        if !self.is_input_in_src_range(src) {
            return src;
        }
        let offset = self.offset_of(src);

        self.dest.start + offset
    }
}

fn parse_vector_of_maps_from_string_vector(input: &Vec<&str>) -> Vec<Map> {
    input.into_iter().map(|l| Map::from(l)).collect()
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
    fn from(input: &Vec<&str>) -> SeedLocation {
        let seeds = parse_seeds(input[0]);

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

    fn map_seed_to_location(self: &Self, seed: u64) -> u64 {
        print!("Seed {} | ", seed);

        let mut soil = seed;
        for map in &self.seed_to_soil {
            if map.is_input_in_src_range(soil) {
                soil = map.map_src_to_dest(soil);
                break;
            }
        }
        print!("Soil {} | ", soil);

        let mut fertilizer = soil;
        for map in &self.soil_to_fertilizer {
            if map.is_input_in_src_range(fertilizer) {
                fertilizer = map.map_src_to_dest(fertilizer);
                break;
            }
        }
        print!("Fertilizer {} | ", fertilizer);

        let mut water = fertilizer;
        for map in &self.fertilizer_to_water {
            if map.is_input_in_src_range(water) {
                water = map.map_src_to_dest(water);
                break;
            }
        }
        print!("Water {} | ", water);

        let mut light = water;
        for map in &self.water_to_light {
            if map.is_input_in_src_range(light) {
                light = map.map_src_to_dest(light);
                break;
            }
        }
        print!("Light {} | ", light);

        let mut temperature = light;
        for map in &self.light_to_temperature {
            if map.is_input_in_src_range(temperature) {
                temperature = map.map_src_to_dest(temperature);
                break;
            }
        }
        print!("Temperature {} | ", temperature);

        let mut humidity = temperature;
        for map in &self.temperature_to_humidity {
            if map.is_input_in_src_range(humidity) {
                humidity = map.map_src_to_dest(humidity);
                break;
            }
        }
        print!("Humidity {} | ", humidity);

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
    fn can_parse_seeds_from_string() {
        let input = "seeds: 63 123 6243 123";
        let seeds = parse_seeds(input);

        assert_eq!(seeds, vec![63, 123, 6243, 123]);

        let input = "seeds: 152 643 742 302 910 120";
        let seeds = parse_seeds(input);

        assert_eq!(seeds, vec![152, 643, 742, 302, 910, 120]);
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
}
