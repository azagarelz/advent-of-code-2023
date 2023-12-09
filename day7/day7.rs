use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const J_LABEL_DIGIT_PART_2: u32 = 1;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_hand_type(card_pairs: &HashMap<u32, u32>) -> u32 {
    let mut card_kind: u32 = 0;

    if card_pairs.len() == 5 {
        // High card
        card_kind = 7;
    }
    if card_pairs.len() == 1 {
        // Five of a kind
        card_kind = 1;
    } else if card_pairs.len() == 2 {
        for value in card_pairs.values() {
            if *value == 4 {
                // Four of a kind
                card_kind = 2;
                break;
            } else {
                // Full house
                card_kind = 3;
            }
        }
    } else if card_pairs.len() == 3 {
        for value in card_pairs.values() {
            if *value == 3 {
                // Three of a kind
                card_kind = 4;
                break;
            } else {
                // Two pair
                card_kind = 5;
            }
        }
    } else if card_pairs.len() == 4 {
        // One pair
        card_kind = 6;
    }

    card_kind
}

fn get_hand_type_part1(hand: &Vec<u32>) -> u32 {
    let mut card_pairs: HashMap<u32, u32> = HashMap::new();

    for label in hand {
        card_pairs
            .entry(*label)
            .and_modify(|time| *time += 1)
            .or_insert(1);
    }

    get_hand_type(&card_pairs)
}

fn get_digit_from_label_part1(label: &char) -> u32 {
    if label.is_digit(10) {
        return label.to_digit(10).unwrap();
    } else {
        match label {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Malformed file"),
        }
    }
}

fn get_label_from_digit_part1(label: &u32) -> String {
    if *label < 10 {
        return label.to_string();
    } else {
        match label {
            10 => "T".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            14 => "A".to_string(),
            _ => panic!("Malformed file"),
        }
    }
}

fn get_hand_type_part2(hand: &Vec<u32>) -> u32 {
    let mut card_pairs: HashMap<u32, u32> = HashMap::new();

    for label in hand {
        card_pairs
            .entry(*label)
            .and_modify(|time| *time += 1)
            .or_insert(1);
    }

    let joker_card_count = card_pairs.remove(&J_LABEL_DIGIT_PART_2);

    if joker_card_count.is_some() {
        let mut best_key_to_increment_key: u32 = J_LABEL_DIGIT_PART_2;
        let mut best_key_to_increment_value = 0;

        for (key, value) in card_pairs.iter_mut() {
            if *value > best_key_to_increment_value {
                best_key_to_increment_key = *key;
                best_key_to_increment_value = *value
            }
        }

        card_pairs
            .entry(best_key_to_increment_key)
            .and_modify(|value| *value += joker_card_count.unwrap())
            .or_insert(joker_card_count.unwrap());
    }

    get_hand_type(&card_pairs)
}

fn get_digit_from_label_part2(label: &char) -> u32 {
    if label.is_digit(10) {
        return label.to_digit(10).unwrap();
    } else {
        match label {
            'T' => 10,
            'J' => J_LABEL_DIGIT_PART_2,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Malformed file"),
        }
    }
}

fn get_label_from_digit_part2(label: &u32) -> String {
    if *label != 1 && *label < 10 {
        return label.to_string();
    } else {
        match label {
            10 => "T".to_string(),
            &J_LABEL_DIGIT_PART_2 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            14 => "A".to_string(),
            _ => panic!("Malformed file"),
        }
    }
}

fn main() {
    let Ok(lines) = read_lines("input.txt") else {
        return;
    };

    let mut hands_part1: Vec<Vec<u32>> = Vec::new();
    let mut bids_map_part1: HashMap<String, u32> = HashMap::new();

    let mut hands_part2: Vec<Vec<u32>> = Vec::new();
    let mut bids_map_part2: HashMap<String, u32> = HashMap::new();

    for line in lines {
        match line {
            Ok(line) => {
                let mut card_bid_split = line.split_ascii_whitespace();

                let mut hand_labels_digits_part1: Vec<u32> = Vec::new();
                let mut hand_labels_digits_part2: Vec<u32> = Vec::new();
                let labels = card_bid_split.next().unwrap();

                for hand_label_char in labels.chars() {
                    hand_labels_digits_part1.push(get_digit_from_label_part1(&hand_label_char));
                    hand_labels_digits_part2.push(get_digit_from_label_part2(&hand_label_char));
                }

                hands_part1.push(hand_labels_digits_part1);
                hands_part2.push(hand_labels_digits_part2);

                let card_bid_split_part = card_bid_split.next().unwrap();

                bids_map_part1.insert(
                    labels.to_string(),
                    card_bid_split_part.parse::<u32>().unwrap(),
                );
                bids_map_part2.insert(
                    labels.to_string(),
                    card_bid_split_part.parse::<u32>().unwrap(),
                );
            }
            _ => panic!("Malformed file"),
        }
    }

    hands_part1.sort_unstable_by(|a, b: &Vec<u32>| {
        let mut hand_comp_result = get_hand_type_part1(&b).cmp(&get_hand_type_part1(&a));

        if hand_comp_result.is_eq() {
            hand_comp_result = a[0]
                .cmp(&b[0])
                .then(a[1].cmp(&b[1]))
                .then(a[2].cmp(&b[2]))
                .then(a[3].cmp(&b[3]))
                .then(a[4].cmp(&b[4]));
        }

        hand_comp_result
    });

    hands_part2.sort_unstable_by(|a, b: &Vec<u32>| {
        let mut hand_comp_result = get_hand_type_part2(&b).cmp(&get_hand_type_part2(&a));

        if hand_comp_result.is_eq() {
            hand_comp_result = a[0]
                .cmp(&b[0])
                .then(a[1].cmp(&b[1]))
                .then(a[2].cmp(&b[2]))
                .then(a[3].cmp(&b[3]))
                .then(a[4].cmp(&b[4]));
        }

        hand_comp_result
    });

    let mut total_winning_part_1: u32 = 0;
    let mut total_winning_part_2: u32 = 0;

    for (index, hand) in hands_part1.iter().enumerate() {
        let key = hand.iter().fold("".to_string(), |acc, &value| {
            acc + get_label_from_digit_part1(&value).as_str()
        });

        total_winning_part_1 += (*bids_map_part1.get(&key).unwrap()) as u32 * (index + 1) as u32;

        // println!(
        //     "{} {} * {}",
        //     key,
        //     bids_map_part1.get(&key).unwrap(),
        //     index + 1
        // );
    }

    for (index, hand) in hands_part2.iter().enumerate() {
        let key = hand.iter().fold("".to_string(), |acc, &value| {
            acc + get_label_from_digit_part2(&value).as_str()
        });

        total_winning_part_2 += (*bids_map_part2.get(&key).unwrap()) as u32 * (index + 1) as u32;

        // println!(
        //     "{} {} * {}",
        //     key,
        //     bids_map_part2.get(&key).unwrap(),
        //     index + 1
        // );
    }

    println!("part 1: {}", total_winning_part_1);
    println!("part 2: {}", total_winning_part_2);
}
