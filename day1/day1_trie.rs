use std::char;
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

struct Node<'a> {
    children: HashMap<char, Node<'a>>,
    is_end_of_word: bool,
    value: &'a str,
}

impl<'a> Node<'a> {
    fn new() -> Node<'a> {
        Node {
            children: HashMap::new(),
            is_end_of_word: false,
            value: "",
        }
    }

    fn add_word(&mut self, word: &str, value: &'a str) {
        let mut current_node = self;

        for letter in word.chars() {
            current_node = current_node.children.entry(letter).or_insert(Node::new());
        }

        current_node.is_end_of_word = true;
        current_node.value = value;
    }

    fn get_words(&self, word: &str) -> Option<String> {
        let mut word_result: String = String::new();
        let root = self;
        let mut current_node = self;

        let mut i = 0;
        let mut j = 0;

        while i < word.len() && j < word.len() {
            let char = word.chars().nth(j);
            let next = current_node.children.get(&char.unwrap());

            match next {
                Some(next) => {
                    j += 1;
                    current_node = next;
                    if next.is_end_of_word {
                        current_node = root;
                        word_result.push_str(next.value);
                        i += 1;
                        j = i;
                    }
                }
                None => {
                    current_node = root;
                    i += 1;
                    j = i;
                }
            }
        }

        if word_result.len() == 0 {
            return None;
        }

        Some(word_result)
    }
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut sum_of_calibration_value_part_1 = 0;
        let mut sum_of_calibration_value_part_2 = 0;

        let mut root_part1 = Node::new();
        root_part1.add_word("1", "1");
        root_part1.add_word("2", "2");
        root_part1.add_word("3", "3");
        root_part1.add_word("4", "4");
        root_part1.add_word("5", "5");
        root_part1.add_word("6", "6");
        root_part1.add_word("7", "7");
        root_part1.add_word("8", "8");
        root_part1.add_word("9", "9");

        let mut root_part2 = Node::new();
        root_part2.add_word("one", "1");
        root_part2.add_word("two", "2");
        root_part2.add_word("three", "3");
        root_part2.add_word("four", "4");
        root_part2.add_word("five", "5");
        root_part2.add_word("six", "6");
        root_part2.add_word("seven", "7");
        root_part2.add_word("eight", "8");
        root_part2.add_word("nine", "9");
        root_part2.add_word("1", "1");
        root_part2.add_word("2", "2");
        root_part2.add_word("3", "3");
        root_part2.add_word("4", "4");
        root_part2.add_word("5", "5");
        root_part2.add_word("6", "6");
        root_part2.add_word("7", "7");
        root_part2.add_word("8", "8");
        root_part2.add_word("9", "9");

        for line in lines {
            match line {
                Ok(line) => {
                    let values_part1 = root_part1.get_words(&line);
                    if values_part1.is_none() {
                        continue;
                    }
                    let chars_part1: Vec<char> = values_part1.unwrap().chars().collect();
                    sum_of_calibration_value_part_1 += format!(
                        "{}{}",
                        chars_part1.first().unwrap(),
                        chars_part1.last().unwrap()
                    )
                    .parse::<u32>()
                    .unwrap();

                    let values_part2 = root_part2.get_words(&line);
                    if values_part2.is_none() {
                        continue;
                    }
                    let chars_part2: Vec<char> = values_part2.unwrap().chars().collect();
                    sum_of_calibration_value_part_2 += format!(
                        "{}{}",
                        chars_part2.first().unwrap(),
                        chars_part2.last().unwrap()
                    )
                    .parse::<u32>()
                    .unwrap();
                }
                _ => panic!("Malformed file"),
            }
        }

        println!("part 1: {}", sum_of_calibration_value_part_1);
        println!("part 2: {}", sum_of_calibration_value_part_2);
    }
}
