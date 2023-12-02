use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_cubes_count(cubes_group: &str) -> (u32, u32, u32) {
    let mut red_cubes = 0;
    let mut green_cubes = 0;
    let mut blue_cubes = 0;

    for cube_color_group in cubes_group.split(',') {
        let mut cube_color_group_split = cube_color_group.split_ascii_whitespace();
        let cube_count = cube_color_group_split
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let cube_color = cube_color_group_split.next().unwrap();

        if cube_color == "red" {
            red_cubes = cube_count;
        } else if cube_color == "green" {
            green_cubes = cube_count;
        } else if cube_color == "blue" {
            blue_cubes = cube_count;
        }
    }

    return (red_cubes, green_cubes, blue_cubes);
}

fn get_games_ids_if_valid_part_1(line: &str) -> Option<u32> {
    let mut game_id_rest_split = line.split(':');

    let game_id = game_id_rest_split
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    for cubes_group in game_id_rest_split.next().unwrap().split(';') {
        let (red_cubes, green_cubes, blue_cubes) = get_cubes_count(cubes_group);

        if red_cubes > MAX_RED_CUBES {
            return None;
        } else if green_cubes > MAX_GREEN_CUBES {
            return None;
        } else if blue_cubes > MAX_BLUE_CUBES {
            return None;
        }
    }

    return Some(game_id);
}

fn get_sum_of_games_powers_part_2(line: &str) -> Option<u32> {
    let game_id_rest_split = line.split(':');

    let mut max_red_cubes = 0;
    let mut max_green_cubes = 0;
    let mut max_blue_cubes = 0;

    for cubes_group in game_id_rest_split.last().unwrap().split(';') {
        let (red_cubes, green_cubes, blue_cubes) = get_cubes_count(cubes_group);

        if red_cubes > max_red_cubes {
            max_red_cubes = red_cubes;
        }

        if green_cubes > max_green_cubes {
            max_green_cubes = green_cubes;
        }

        if blue_cubes > max_blue_cubes {
            max_blue_cubes = blue_cubes;
        }
    }

    return Some(max_red_cubes * max_green_cubes * max_blue_cubes);
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut sum_of_valid_game_ids_1 = 0;
        let mut sum_of_valid_game_ids_2 = 0;

        for line in lines {
            match line {
                Ok(line) => {
                    let get_valid_game_id_part_1_result = get_games_ids_if_valid_part_1(&line);

                    if get_valid_game_id_part_1_result.is_some() {
                        sum_of_valid_game_ids_1 =
                            sum_of_valid_game_ids_1 + get_valid_game_id_part_1_result.unwrap();
                    }

                    let get_valid_game_id_part_2_result = get_sum_of_games_powers_part_2(&line);
                    if get_valid_game_id_part_2_result.is_some() {
                        sum_of_valid_game_ids_2 =
                            sum_of_valid_game_ids_2 + get_valid_game_id_part_2_result.unwrap();
                    }
                }
                _ => panic!("Malformed file"),
            }
        }

        println!("part 1: {}", sum_of_valid_game_ids_1);
        println!("part 2: {}", sum_of_valid_game_ids_2);
    }
}
