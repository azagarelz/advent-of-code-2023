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

fn until_is_zzz(node: &str) -> bool {
    node == "ZZZ"
}

fn until_end_with_z(node: &str) -> bool {
    node.ends_with('Z')
}

fn greatest_common_divisor(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        greatest_common_divisor(b, a % b)
    }
}

fn least_common_multiple(a: u64, b: u64) -> u64 {
    a / greatest_common_divisor(a, b) * b
}

fn least_common_multiple_on_vec(numbers: &Vec<u32>) -> u64 {
    numbers
        .iter()
        .fold(1, |a, &b| least_common_multiple(a, b as u64))
}

fn main() {
    let Ok(lines) = read_lines("input.txt") else {
        return;
    };

    let mut directions: Vec<char> = Vec::new();
    let mut left_map: HashMap<String, String> = HashMap::new();
    let mut right_map: HashMap<String, String> = HashMap::new();

    let mut direction_read = false;
    for line in lines {
        match line {
            Ok(line) => {
                if !direction_read {
                    for direction in line.chars() {
                        directions.push(direction);
                    }
                    direction_read = true;
                    continue;
                }

                if line.is_empty() {
                    continue;
                }

                let mut split_headers_from_values = line.split('=');
                let header = split_headers_from_values.next().unwrap().trim_end();

                // println!("header: {}", header);

                let mut next_node_split_part = split_headers_from_values.next().unwrap().split(',');

                let left_direction_next_node = next_node_split_part
                    .next()
                    .unwrap()
                    .split("(")
                    .last()
                    .unwrap()
                    .trim_end();

                let right_direction_next_node = next_node_split_part
                    .next()
                    .unwrap()
                    .split(")")
                    .next()
                    .unwrap()
                    .trim_start();

                left_map.insert(header.to_string(), left_direction_next_node.to_string());
                right_map.insert(header.to_string(), right_direction_next_node.to_string());
            }
            _ => panic!("Malformed file"),
        }
    }

    let mut step_required_to_reach_zzz_part1: u32 = 0;

    let mut current_node = "AAA";
    while !until_is_zzz(current_node) {
        for direction in directions.iter() {
            match direction {
                'L' => current_node = left_map.get(current_node).unwrap(),
                'R' => current_node = right_map.get(current_node).unwrap(),
                _ => panic!("Malformed file"),
            }
            step_required_to_reach_zzz_part1 += 1;
            if until_is_zzz(current_node) {
                break;
            }
        }
    }

    let mut current_nodes: Vec<String> = Vec::new();
    for node_that_end_with_a in left_map.keys() {
        if node_that_end_with_a.ends_with('A') {
            current_nodes.push(node_that_end_with_a.to_string());
        }
    }

    let mut steps_required_to_reach_nodes_end_with_z_vec: Vec<u32> = Vec::new();
    for starting_node in current_nodes {
        let mut current_node = starting_node.clone();
        let mut step_required_to_reach_zzz_part2 = 0;

        while !until_end_with_z(&current_node) {
            for direction in directions.iter() {
                match direction {
                    'L' => current_node = left_map.get(&current_node).unwrap().to_string(),
                    'R' => current_node = right_map.get(&current_node).unwrap().to_string(),
                    _ => panic!("Malformed file"),
                }
                step_required_to_reach_zzz_part2 += 1;

                if until_end_with_z(&current_node) {
                    steps_required_to_reach_nodes_end_with_z_vec
                        .push(step_required_to_reach_zzz_part2);
                    break;
                }
            }
        }
    }

    let steps_required_to_reach_nodes_end_with_z_part2 =
        least_common_multiple_on_vec(&steps_required_to_reach_nodes_end_with_z_vec);

    println!("part 1: {}", step_required_to_reach_zzz_part1);
    println!("part 2: {}", steps_required_to_reach_nodes_end_with_z_part2);
}
