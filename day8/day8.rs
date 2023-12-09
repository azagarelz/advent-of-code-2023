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

struct Node {
    left: String,
    right: String,
}

fn until_is_zzz(node: &str) -> bool {
    node == "ZZZ"
}

fn until_end_with_z(node: &str) -> bool {
    node.ends_with('Z')
}

fn traverse_tree_until(
    tree: &HashMap<String, Node>,
    directions: &Vec<char>,
    starting_node: &str,
    until_function: fn(&str) -> bool,
) -> u32 {
    let mut steps_required = 0;
    let mut current_node = starting_node;

    while !until_function(current_node) {
        for direction in directions.iter() {
            match direction {
                'L' => {
                    current_node = tree.get(current_node).unwrap().left.as_str();
                }
                'R' => {
                    current_node = tree.get(current_node).unwrap().right.as_str();
                }
                _ => panic!("Malformed file"),
            }
            steps_required += 1;
            if until_function(current_node) {
                break;
            }
        }
    }

    steps_required
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
    let mut tree: HashMap<String, Node> = HashMap::new();

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

                tree.insert(
                    header.to_string(),
                    Node {
                        left: left_direction_next_node.to_string(),
                        right: right_direction_next_node.to_string(),
                    },
                );
            }
            _ => panic!("Malformed file"),
        }
    }

    let step_required_to_reach_zzz_part1 =
        traverse_tree_until(&tree, &directions, "AAA", until_is_zzz);

    let mut nodes_that_end_with_a: Vec<String> = Vec::new();
    for node_that_end_with_a in tree.keys() {
        if node_that_end_with_a.ends_with('A') {
            nodes_that_end_with_a.push(node_that_end_with_a.to_string());
        }
    }

    let mut steps_required_to_reach_nodes_end_with_z_vec: Vec<u32> = Vec::new();
    for node_that_end_with_a in nodes_that_end_with_a {
        steps_required_to_reach_nodes_end_with_z_vec.push(traverse_tree_until(
            &tree,
            &directions,
            node_that_end_with_a.as_str(),
            until_end_with_z,
        ))
    }

    let steps_required_to_reach_nodes_end_with_z_part2 =
        least_common_multiple_on_vec(&steps_required_to_reach_nodes_end_with_z_vec);

    println!("part 1: {}", step_required_to_reach_zzz_part1);
    println!("part 2: {}", steps_required_to_reach_nodes_end_with_z_part2);
}
