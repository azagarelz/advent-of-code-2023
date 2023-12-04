use std::collections::HashMap;
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

fn calculate_calibration_value_part_internal(line: &str, expand_word: bool) -> u32 {
    let word_digit_number = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let chars_vector = line.chars().collect::<Vec<_>>();
    let mut numbers: Vec<u32> = Vec::new();

    for (index, char) in chars_vector.iter().enumerate() {
        if char.is_digit(10) {
            numbers.push(char.to_digit(10).unwrap())
        } else if expand_word {
            for (digit_word, digit) in word_digit_number.iter() {
                let to_compare_string =
                    String::from_iter(chars_vector[index..index + digit_word.len()].iter());

                if to_compare_string == *digit_word {
                    numbers.push(digit.to_digit(10).unwrap());
                    break;
                }
            }
        }
    }

    format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
        .parse::<u32>()
        .unwrap()
}

fn calculate_calibration_value_part_1(line: &str) -> u32 {
    calculate_calibration_value_part_internal(&line, false)
}

fn calculate_calibration_value_part_2(line: &str) -> u32 {
    calculate_calibration_value_part_internal(&line, true)
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut sum_of_calibration_value_part_1 = 0;
        let mut sum_of_calibration_value_part_2 = 0;

        for line in lines {
            match line {
                Ok(line) => {
                    sum_of_calibration_value_part_1 += calculate_calibration_value_part_1(&line);
                    sum_of_calibration_value_part_2 += calculate_calibration_value_part_2(&line);
                }
                _ => panic!("Malformed file"),
            }
        }

        println!("part 1: {}", sum_of_calibration_value_part_1);
        println!("part 2: {}", sum_of_calibration_value_part_2);
    }
}
