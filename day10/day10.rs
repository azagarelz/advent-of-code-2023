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

fn can_proceed_west(
    x_index: i32,
    y_index: i32,
    prevous_x_index: i32,
    tales: &Vec<Vec<char>>,
) -> bool {
    let first_validaton = x_index - 1 >= 0 && x_index - 1 != prevous_x_index;
    if !first_validaton {
        return false;
    }
    let current_tale = tales[y_index as usize][x_index as usize];
    let next_tale = tales[y_index as usize][x_index as usize - 1];
    if next_tale == '.' {
        return false;
    }
    else if (current_tale == '-'
        || current_tale == 'J'
        || current_tale == '7'
        || current_tale == 'S')
        && next_tale == 'F'
    {
        return true;
    } else if (current_tale == '-'
        || current_tale == 'J'
        || current_tale == '7'
        || current_tale == 'S')
        && next_tale == 'L'
    {
        return true;
    } else if (current_tale == '-'
        || current_tale == 'J'
        || current_tale == '7'
        || current_tale == 'S')
        && next_tale == '-'
    {
        return true;
    } else if (current_tale == '-' || current_tale == 'J' || current_tale == '7')
        && next_tale == 'S'
    {
        return true;
    } else {
        return false;
    }
}

fn can_proceed_east(
    x_index: i32,
    y_index: i32,
    prevous_x_index: i32,
    tales: &Vec<Vec<char>>,
    line_length: u32,
) -> bool {
    let first_validaton = x_index + 1 != line_length as i32 && x_index + 1 != prevous_x_index;
    if !first_validaton {
        return false;
    }
    let current_tale = tales[y_index as usize][x_index as usize];
    let next_tale = tales[y_index as usize][x_index as usize + 1];
    if next_tale == '.' {
        return false;
    }
    else if (current_tale == '-'
        || current_tale == 'L'
        || current_tale == 'F'
        || current_tale == 'S')
        && next_tale == '-'
    {
        return true;
    } else if (current_tale == '-'
        || current_tale == 'L'
        || current_tale == 'F'
        || current_tale == 'S')
        && next_tale == '7'
    {
        return true;
    } else if (current_tale == '-'
        || current_tale == 'L'
        || current_tale == 'F'
        || current_tale == 'S')
        && next_tale == 'J'
    {
        return true;
    } else if (current_tale == '-' || current_tale == 'L' || current_tale == 'F')
        && next_tale == 'S'
    {
        return true;
    } else {
        return false;
    }
}

fn can_proceed_north(
    x_index: i32,
    y_index: i32,
    prevous_y_index: i32,
    tales: &Vec<Vec<char>>,
) -> bool {
    let first_validaton = y_index - 1 >= 0 && y_index - 1 != prevous_y_index;
    if !first_validaton {
        return false;
    }
    let current_tale = tales[y_index as usize][x_index as usize];
    let next_tale = tales[y_index as usize - 1][x_index as usize];
    // println!("current_tale: {:?}", current_tale);
    // println!("next_tale: {:?}", next_tale);

    if next_tale == '.' {
        return false;
    } else if (current_tale == '|'
        || current_tale == 'L'
        || current_tale == 'J'
        || current_tale == 'S')
        && next_tale == '|'
    {
        return true;
    } else if (current_tale == '|'
        || current_tale == 'L'
        || current_tale == 'J'
        || current_tale == 'S')
        && next_tale == '7'
    {
        return true;
    } else if (current_tale == '|'
        || current_tale == 'L'
        || current_tale == 'J'
        || current_tale == 'S')
        && next_tale == 'F'
    {
        return true;
    } else if (current_tale == '|' || current_tale == 'L' || current_tale == 'J')
        && next_tale == 'S'
    {
        return true;
    } else {
        return false;
    }
}

fn can_proceed_south(
    x_index: i32,
    y_index: i32,
    prevous_y_index: i32,
    tales: &Vec<Vec<char>>,
    line_length: u32,
) -> bool {
    let first_validaton = y_index + 1 != line_length as i32 && y_index + 1 != prevous_y_index;
    if !first_validaton {
        return false;
    }
    let current_tale = tales[y_index as usize][x_index as usize];
    let next_tale = tales[y_index as usize + 1][x_index as usize];
    // println!("current_tale: {:?}", current_tale);
    // println!("next_tale: {:?}", next_tale);
    if next_tale == '.' {
        return false;
    } else if (current_tale == '|'
        || current_tale == '7'
        || current_tale == 'F'
        || current_tale == 'S')
        && next_tale == '|'
    {
        return true;
    } else if (current_tale == '|'
        || current_tale == '7'
        || current_tale == 'F'
        || current_tale == 'S')
        && next_tale == 'L'
    {
        return true;
    } else if (current_tale == '|'
        || current_tale == '7'
        || current_tale == 'F'
        || current_tale == 'S')
        && next_tale == 'J'
    {
        return true;
    } else if (current_tale == '|' || current_tale == '7' || current_tale == 'F')
        && next_tale == 'S'
    {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let Ok(lines) = read_lines("input_test.txt") else {
        return;
    };

    let mut tales: Vec<Vec<char>> = Vec::new();
    let mut start_x = 0;
    let mut start_y = 0;

    for (y_index, line) in lines.enumerate() {
        match line {
            Ok(line) => {
                let mut line_tales: Vec<char> = Vec::new();

                line.chars().enumerate().for_each(|(x_index, tale)| {
                    match tale {
                        '|' | '-' | 'L' | 'J' | 'F' | '7' | '.' => (),
                        'S' => {
                            start_x = x_index;
                            start_y = y_index;
                        }
                        _ => panic!("Malformed file"),
                    }
                    line_tales.push(tale);
                });

                tales.push(line_tales);
            }
            _ => panic!("Malformed file"),
        }
    }

    let mut path: Vec<char> = Vec::new();
    let mut x_index: usize = start_x;
    let mut y_index: usize = start_y;
    let mut prevous_x_index = start_x as i32;
    let mut prevous_y_index = start_y as i32;
    loop {
        let next_tale: char;
        let x_index_i32 = x_index as i32;
        let y_index_i32 = y_index as i32;

        if can_proceed_west(x_index_i32, y_index_i32, prevous_x_index, &tales) {
            next_tale = tales[y_index][x_index - 1];
            x_index -= 1;
            // println!("W");
        } else if can_proceed_east(
            x_index_i32,
            y_index_i32,
            prevous_x_index,
            &tales,
            tales[y_index].len() as u32,
        ) {
            next_tale = tales[y_index][x_index + 1];
            x_index += 1;
            // println!("E");
        } else if can_proceed_north(x_index_i32, y_index_i32, prevous_y_index, &tales) {
            next_tale = tales[y_index - 1][x_index];
            y_index -= 1;
            // println!("N");
        } else if can_proceed_south(
            x_index_i32,
            y_index_i32,
            prevous_y_index,
            &tales,
            tales.len() as u32,
        ) {
            next_tale = tales[y_index + 1][x_index];
            y_index += 1;
            // println!("S");
        } else {
            println!(
                "North/Current {:?} {:?}",
                tales[y_index - 1][x_index],
                tales[y_index][x_index]
            );
            println!(
                "South/Current {:?} {:?}",
                tales[y_index + 1][x_index],
                tales[y_index][x_index]
            );
            println!(
                "East/Current {:?} {:?}",
                tales[y_index][x_index + 1],
                tales[y_index][x_index]
            );
            println!(
                "West/Current {:?} {:?}",
                tales[y_index][x_index - 1],
                tales[y_index][x_index]
            );
            println!("path.len: {:?}", path.len());
            panic!("Boh");
        }

        path.push(next_tale);

        prevous_x_index = x_index_i32;
        prevous_y_index = y_index_i32;

        if next_tale == 'S' {
            // println!("{:?}", path);
            break;
        }
    }

    println!("part 1: {}", path.len() / 2);
}
