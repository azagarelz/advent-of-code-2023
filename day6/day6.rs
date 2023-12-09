use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let Ok(lines) = read_lines("input.txt") else {
        return;
    };

    let mut race_timings: Vec<u64> = Vec::new();
    let mut race_distances: Vec<u64> = Vec::new();

    let mut race_vector: &mut Vec<u64>;
    for line in lines {
        match line {
            Ok(line) => {
                let mut split_headers_from_values = line.split(':');
                let header = split_headers_from_values.next().unwrap();

                println!("header: {}", header);

                match header {
                    "Time" => {
                        race_vector = &mut race_timings;
                    }
                    "Distance" => {
                        race_vector = &mut race_distances;
                    }
                    _ => panic!("Malformed file"),
                }

                for value in split_headers_from_values
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                {
                    if value.is_empty() {
                        continue;
                    }
                    race_vector.push(value.parse::<u64>().unwrap());
                }
            }
            _ => panic!("Malformed file"),
        }
    }

    race_timings.push(
        race_timings
            .iter()
            .fold("".to_string(), |acc, &value| acc + &value.to_string())
            .parse::<u64>()
            .unwrap(),
    );

    race_distances.push(
        race_distances
            .iter()
            .fold("".to_string(), |acc, &value| acc + &value.to_string())
            .parse::<u64>()
            .unwrap(),
    );

    let mut number_of_ways_you_can_beat_the_record_part_1: Option<u64> = None;
    let mut number_of_ways_you_can_beat_the_record_part_2: u64 = 0;

    for (index, race_timing) in race_timings.iter().enumerate() {
        let mut possibile_solution: u64 = 0;

        let race_timing = *race_timing;
        for button_hold_timing in 1..race_timing - 1 {
            if button_hold_timing * (race_timing - button_hold_timing) > race_distances[index] {
                possibile_solution += 1;
            }
        }

        if race_timings.len() == index + 1 {
            number_of_ways_you_can_beat_the_record_part_2 = possibile_solution;
        } else {
            if number_of_ways_you_can_beat_the_record_part_1.is_none() {
                number_of_ways_you_can_beat_the_record_part_1 = Some(possibile_solution);
            } else {
                number_of_ways_you_can_beat_the_record_part_1 = Some(
                    number_of_ways_you_can_beat_the_record_part_1.unwrap() * possibile_solution,
                );
            }
        }
    }

    println!(
        "part 1: {:?}",
        number_of_ways_you_can_beat_the_record_part_1.unwrap()
    );
    println!(
        "part 2: {:?}",
        number_of_ways_you_can_beat_the_record_part_2
    );
}
