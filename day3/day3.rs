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

fn is_valid_symbol(char: char) -> bool {
    char != '.' && char.is_ascii_punctuation()
}

fn is_gear(char: char) -> bool {
    char == '*'
}

fn get_adjacent_number_start_end(
    engine_schema: &Vec<Vec<char>>,
    max_column: usize,
    row_index: usize,
    column_index: usize,
) -> (u32, u32) {
    let mut number_parsing_column_index = column_index + 1;

    while number_parsing_column_index < max_column
        && engine_schema[row_index][number_parsing_column_index].is_digit(10)
    {
        number_parsing_column_index += 1;
    }

    let mut number_parsing_column_loop_index = (column_index - 1) as i32;
    while number_parsing_column_loop_index >= 0
        && engine_schema[row_index][number_parsing_column_loop_index as usize].is_digit(10)
    {
        number_parsing_column_loop_index -= 1;
    }

    return (
        (number_parsing_column_loop_index + 1) as u32,
        (number_parsing_column_index - 1) as u32,
    );
}

fn parse_number_from_indexes(row: &Vec<char>, start_index: u32, end_index: u32) -> u32 {
    // println!("start_index: {:?}", start_index);
    // println!("end_index: {:?}", end_index);
    // println!("row: {:?}", row);

    // println!("{:?}", row[(start_index) as usize]);
    // println!("{:?}", row[(end_index) as usize]);

    row[start_index as usize..(end_index + 1) as usize]
        .into_iter()
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

fn get_adjacent_numbers(
    engine_schema: &Vec<Vec<char>>,
    max_row: usize,
    max_column: usize,
    row_index: usize,
    column_index: usize,
) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    let mut previous_number_end: u32 = 0;

    if row_index != 0
        && column_index != 0
        && engine_schema[row_index - 1][column_index - 1].is_digit(10)
    {
        let (number_start, number_end) = get_adjacent_number_start_end(
            engine_schema,
            max_column,
            row_index - 1,
            column_index - 1,
        );
        previous_number_end = number_end;

        numbers.push(parse_number_from_indexes(
            &engine_schema[row_index - 1],
            number_start,
            number_end,
        ));
    }

    if row_index != 0
        && (previous_number_end as usize) < column_index
        && engine_schema[row_index - 1][column_index].is_digit(10)
    {
        let (number_start, number_end) =
            get_adjacent_number_start_end(engine_schema, max_column, row_index - 1, column_index);
        previous_number_end = number_end;

        numbers.push(parse_number_from_indexes(
            &engine_schema[row_index - 1],
            number_start,
            number_end,
        ));
    }

    if row_index != 0
        && column_index != max_column
        && (previous_number_end as usize) < column_index
        && engine_schema[row_index - 1][column_index + 1].is_digit(10)
    {
        let (number_start, number_end) = get_adjacent_number_start_end(
            engine_schema,
            max_column,
            row_index - 1,
            column_index + 1,
        );

        numbers.push(parse_number_from_indexes(
            &engine_schema[row_index - 1],
            number_start,
            number_end,
        ));
    }

    if column_index != max_column && engine_schema[row_index][column_index + 1].is_digit(10) {
        let (number_start, number_end) =
            get_adjacent_number_start_end(engine_schema, max_column, row_index, column_index + 1);

        numbers.push(parse_number_from_indexes(
            &engine_schema[row_index],
            number_start,
            number_end,
        ));
    }

    if column_index != 0 && engine_schema[row_index][column_index - 1].is_digit(10) {
        let (number_start, number_end) =
            get_adjacent_number_start_end(engine_schema, max_column, row_index, column_index - 1);

        numbers.push(parse_number_from_indexes(
            &engine_schema[row_index],
            number_start,
            number_end,
        ));
    }

    previous_number_end = 0;

    if row_index != max_row
        && column_index != 0
        && engine_schema[row_index + 1][column_index - 1].is_digit(10)
    {
        let (number_start, number_end) = get_adjacent_number_start_end(
            engine_schema,
            max_column,
            row_index + 1,
            column_index - 1,
        );
        previous_number_end = number_end;

        numbers.push(parse_number_from_indexes(
            &engine_schema[row_index + 1],
            number_start,
            number_end,
        ));
    }

    if row_index != max_row
        && (previous_number_end as usize) < column_index
        && engine_schema[row_index + 1][column_index].is_digit(10)
    {
        let (number_start, number_end) =
            get_adjacent_number_start_end(engine_schema, max_column, row_index + 1, column_index);
        previous_number_end = number_end;

        numbers.push(parse_number_from_indexes(
            &engine_schema[row_index + 1],
            number_start,
            number_end,
        ));
    }

    if row_index != max_row
        && column_index != max_column
        && (previous_number_end as usize) < column_index
        && engine_schema[row_index + 1][column_index + 1].is_digit(10)
    {
        let (number_start, number_end) = get_adjacent_number_start_end(
            engine_schema,
            max_column,
            row_index + 1,
            column_index + 1,
        );

        numbers.push(parse_number_from_indexes(
            &engine_schema[row_index + 1],
            number_start,
            number_end,
        ));
    }

    numbers
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let mut sum_of_adjacent_numbers_1 = 0;
        let mut sum_of_adjacent_numbers_2 = 0;

        let mut engine_schema: Vec<Vec<char>> = Vec::new();
        for line in lines {
            match line {
                Ok(line) => {
                    engine_schema.push(line.chars().collect::<Vec<char>>());
                }
                _ => panic!("Malformed file"),
            }
        }

        for (row, chars) in engine_schema.iter().enumerate() {
            for (column, char) in chars.iter().enumerate() {
                if is_valid_symbol(*char) {
                    let numbers = get_adjacent_numbers(
                        &engine_schema,
                        engine_schema.len(),
                        chars.len(),
                        row,
                        column,
                    );

                    // println!("numbers: {:?}", numbers);

                    for number in numbers.iter() {
                        sum_of_adjacent_numbers_1 += number;
                    }
                }

                if is_gear(*char) {
                    let numbers = get_adjacent_numbers(
                        &engine_schema,
                        engine_schema.len(),
                        chars.len(),
                        row,
                        column,
                    );

                    // println!("numbers: {:?}", numbers);

                    if numbers.len() == 2 {
                        sum_of_adjacent_numbers_2 += numbers[0] * numbers[1];
                    }
                }
            }
        }

        println!("part 1: {}", sum_of_adjacent_numbers_1);
        println!("part 2: {}", sum_of_adjacent_numbers_2);
    }
}
