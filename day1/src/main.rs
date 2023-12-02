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

fn calculate_calibration_value_part_1(line: &str) -> i32 {
    let numbers = line
        .chars()
        .filter_map(|x| x.to_digit(10))
        .collect::<Vec<_>>();

    return format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
        .parse::<i32>()
        .unwrap();
}

fn calculate_calibration_value_part_2(line: &str) -> i32 {
    let line_with_replace = line
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");
    return calculate_calibration_value_part_1(&line_with_replace);
}

fn main() {
    if let Ok(lines) = read_lines("src/input") {
        let mut sum_of_calibration_value_part_1 = 0;
        let mut sum_of_calibration_value_part_2 = 0;

        for line in lines {
            match line {
                Ok(line) => {
                    sum_of_calibration_value_part_1 =
                        sum_of_calibration_value_part_1 + calculate_calibration_value_part_1(&line);

                    sum_of_calibration_value_part_2 =
                        sum_of_calibration_value_part_2 + calculate_calibration_value_part_2(&line);
                }
                _ => panic!("Malformed file"),
            }
        }

        println!("{}", sum_of_calibration_value_part_1);
        println!("{}", sum_of_calibration_value_part_2);
    }
}
