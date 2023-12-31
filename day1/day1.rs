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

fn calculate_calibration_value_part_1(line: &str) -> u32 {
    let numbers = line
        .chars()
        .filter_map(|x| x.to_digit(10))
        .collect::<Vec<_>>();

    format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
        .parse::<u32>()
        .unwrap()
}

fn calculate_calibration_value_part_2(line: &str) -> u32 {
    let line_with_replace = line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    calculate_calibration_value_part_1(&line_with_replace)
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
