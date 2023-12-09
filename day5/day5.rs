use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_next_mapping_key(current_source_mappings: &Vec<Vec<u64>>, current_mapping_key: u64) -> u64 {
    for current_source_mapping in current_source_mappings {
        let source_range_start = current_source_mapping[1];
        let ranges_lenght = current_source_mapping[2];

        if current_mapping_key >= source_range_start
            && current_mapping_key < source_range_start + ranges_lenght
        {
            return current_source_mapping[0] - (source_range_start - current_mapping_key);
        }
    }
    return current_mapping_key;
}

fn main() {
    let Ok(lines) = read_lines("input.txt") else {
        return;
    };

    let mut seeds: Vec<u64> = Vec::new();
    let mut seeds_part_2: Vec<Vec<u64>> = Vec::new();
    let mut seed_to_soil: Vec<Vec<u64>> = Vec::new();
    let mut soil_to_fertilizer: Vec<Vec<u64>> = Vec::new();
    let mut fertilizer_to_water: Vec<Vec<u64>> = Vec::new();
    let mut water_to_light: Vec<Vec<u64>> = Vec::new();
    let mut light_to_temperature: Vec<Vec<u64>> = Vec::new();
    let mut temperature_to_humidity: Vec<Vec<u64>> = Vec::new();
    let mut humidity_to_location: Vec<Vec<u64>> = Vec::new();

    let mut current_map = &mut seed_to_soil;

    for line in lines {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                let mut line_split = line.split(':');
                match line_split.next().unwrap() {
                    "seeds" => {
                        for seed in line_split.last().unwrap().split(' ') {
                            if seed.is_empty() {
                                continue;
                            }
                            seeds.push(seed.parse::<u64>().unwrap());

                            if seeds_part_2.is_empty() || seeds_part_2.last().unwrap().len() == 2 {
                                seeds_part_2.push(Vec::new());
                            }

                            seeds_part_2
                                .last_mut()
                                .unwrap()
                                .push(seed.parse::<u64>().unwrap());
                        }
                    }
                    "seed-to-soil map" => {
                        continue;
                    }
                    "soil-to-fertilizer map" => {
                        current_map = &mut soil_to_fertilizer;
                    }
                    "fertilizer-to-water map" => {
                        current_map = &mut fertilizer_to_water;
                    }
                    "water-to-light map" => {
                        current_map = &mut water_to_light;
                    }
                    "light-to-temperature map" => {
                        current_map = &mut light_to_temperature;
                    }
                    "temperature-to-humidity map" => {
                        current_map = &mut temperature_to_humidity;
                    }
                    "humidity-to-location map" => {
                        current_map = &mut humidity_to_location;
                    }
                    _ => {
                        let mut map_split = line.split(' ');

                        let destination_range_start =
                            map_split.next().unwrap().parse::<u64>().unwrap();
                        let source_range_start = map_split.next().unwrap().parse::<u64>().unwrap();
                        let range_length = map_split.next().unwrap().parse::<u64>().unwrap();

                        let mut range_vec = Vec::new();
                        range_vec.push(destination_range_start);
                        range_vec.push(source_range_start);
                        range_vec.push(range_length);
                        current_map.push(range_vec);
                    }
                }
            }
            _ => panic!("Malformed file"),
        }
    }

    println!("seed_part_2: {:?}", seeds_part_2);

    let mut min_location_part_1: Option<u64> = None;
    let mut min_location_part_2: Option<u64> = None;

    for seed in seeds {
        let soil: u64 = get_next_mapping_key(&seed_to_soil, seed);
        let fertilizer: u64 = get_next_mapping_key(&soil_to_fertilizer, soil);
        let water: u64 = get_next_mapping_key(&fertilizer_to_water, fertilizer);
        let light: u64 = get_next_mapping_key(&water_to_light, water);
        let temperature: u64 = get_next_mapping_key(&light_to_temperature, light);
        let humidity: u64 = get_next_mapping_key(&temperature_to_humidity, temperature);
        let location: u64 = get_next_mapping_key(&humidity_to_location, humidity);

        if min_location_part_1.is_none() || location < min_location_part_1.unwrap() {
            min_location_part_1 = Some(location);
        }

        println!("Seed {}, soil {}, fertilizer {}, water {}, light {}, temperature {}, humidity {}, location {}.", seed, soil, fertilizer, water, light, temperature, humidity, location);
    }

    let mut ranges_done: Vec<Range<u64>> = Vec::new();
    seeds_part_2.sort_by(|a, b| {
        // Compare the sum of the first two elements of each vector in descending order
        (b[0] + b[1]).cmp(&(a[0] + a[1]))
    });

    for seed_range in seeds_part_2 {
        for range in ranges_done.iter() {
            if range.contains(&(seed_range[0] as u64)) && range.contains(&(seed_range[0] + seed_range[1])) {
                continue;
            }
        }
        for seed_number in seed_range[0]..seed_range[0] + seed_range[1] {
            // println!("Seed number {}", seed_number);
            // for range in ranges_done.iter() {
            //     if range.contains(&seed_number) {
            //         continue;
            //     }
            // }
            let soil: u64 = get_next_mapping_key(&seed_to_soil, seed_number);
            let fertilizer: u64 = get_next_mapping_key(&soil_to_fertilizer, soil);
            let water: u64 = get_next_mapping_key(&fertilizer_to_water, fertilizer);
            let light: u64 = get_next_mapping_key(&water_to_light, water);
            let temperature: u64 = get_next_mapping_key(&light_to_temperature, light);
            let humidity: u64 = get_next_mapping_key(&temperature_to_humidity, temperature);
            let location: u64 = get_next_mapping_key(&humidity_to_location, humidity);

            if min_location_part_2.is_none() || location < min_location_part_2.unwrap() {
                min_location_part_2 = Some(location);
            }
            // println!("Seed {}, soil {}, fertilizer {}, water {}, light {}, temperature {}, humidity {}, location {}.", seed_number, soil, fertilizer, water, light, temperature, humidity, location);
        }
        ranges_done.push(seed_range[0]..seed_range[0] + seed_range[1]);
    }

    println!("part 1: {}", min_location_part_1.unwrap());
    println!("part 2: {}", min_location_part_2.unwrap());
}
