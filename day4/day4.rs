use std::collections::{HashMap, HashSet};
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

fn get_card_id(card_id_string_part: &str) -> u32 {
    card_id_string_part
        .split(' ')
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

fn parse_card_numbers(line: &str) -> HashSet<u32> {
    let mut numbers: HashSet<u32> = HashSet::new();
    for number in line.split(" ") {
        if number.is_empty() {
            continue;
        }
        numbers.insert(number.parse::<u32>().unwrap());
    }
    numbers
}

fn calculate_winning_numbers(numbers_string_part: &str) -> u32 {
    let mut numbers_parts_split: std::str::Split<'_, char> = numbers_string_part.split('|');

    parse_card_numbers(numbers_parts_split.next().unwrap())
        .intersection(&parse_card_numbers(numbers_parts_split.next().unwrap()))
        .count() as u32
}


// fn calculate_total_card(
//     cards: &Vec<String>,
//     card_index: u32,
//     winning_numbers: u32,
//     calculate_winning_cache: &mut HashMap<u32, u32>,
// ) -> u32 {
//     let mut total_cards = winning_numbers;

//     for card_index_below in
//         ((card_index + 1) as usize)..((card_index + 1) as usize + winning_numbers as usize)
//     {
//         let card_string_split: std::str::Split<'_, char> = cards[card_index_below].split(':');

//         let winning_numbers = *calculate_winning_cache
//             .entry(card_index_below as u32)
//             .or_insert(calculate_winning_numbers(card_string_split.last().unwrap()));

//         if winning_numbers == 0 {
//             continue;
//         }

//         total_cards += calculate_total_card(
//             cards,
//             card_index_below as u32,
//             winning_numbers,
//             calculate_winning_cache,
//         );
//     }

//     total_cards
// }

fn calculate_total_card(
    cards: &Vec<String>,
    card_index: u32,
    winning_numbers: u32,
    calculate_winning_cache: &mut HashMap<u32, u32>,
    total_cards: u32, // Accumulator parameter
) -> u32 {
    let mut updated_total_cards = total_cards + winning_numbers;

    for card_index_below in
        ((card_index + 1) as usize)..((card_index + 1) as usize + winning_numbers as usize)
    {
        let card_string_split: std::str::Split<'_, char> = cards[card_index_below].split(':');

        let winning_numbers = *calculate_winning_cache
            .entry(card_index_below as u32)
            .or_insert(calculate_winning_numbers(card_string_split.last().unwrap()));

        if winning_numbers == 0 {
            continue;
        }

        updated_total_cards = calculate_total_card(
            cards,
            card_index_below as u32,
            winning_numbers,
            calculate_winning_cache,
            updated_total_cards,
        );
    }

    updated_total_cards
}

fn main() {
    let Ok(lines) = read_lines("input.txt") else {
        return;
    };

    let mut sum_of_card_points_part_1 = 0;
    let mut sum_of_total_cards_part_2 = 0;

    let mut cards: Vec<String> = Vec::new();

    for line in lines {
        match line {
            Ok(line) => {
                cards.push(line);
            }
            _ => panic!("Malformed file"),
        }
    }

    let mut calculate_winning_cache: HashMap<u32, u32> = HashMap::new();
    for line in cards.as_slice() {
        sum_of_total_cards_part_2 += 1;

        let mut card_string_split = line.split(':');
        let card_id: u32 = get_card_id(card_string_split.next().unwrap());
        let card_index = card_id - 1;

        let winning_numbers = *calculate_winning_cache
            .entry(card_index)
            .or_insert(calculate_winning_numbers(card_string_split.next().unwrap()));

        if winning_numbers == 0 {
            continue;
        }

        sum_of_card_points_part_1 += (2 as u32).pow(winning_numbers - 1);
        sum_of_total_cards_part_2 = calculate_total_card(
            &cards,
            card_index,
            winning_numbers,
            &mut calculate_winning_cache,
            sum_of_total_cards_part_2,
        );
    }

    println!("part 1: {}", sum_of_card_points_part_1);
    println!("part 2: {}", sum_of_total_cards_part_2);
}
