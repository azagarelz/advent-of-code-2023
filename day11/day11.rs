use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
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

fn shortest_path(
    cosmos: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let mut distances = vec![vec![usize::MAX; cosmos[0].len()]; cosmos.len()];
    let mut queue: BinaryHeap<Reverse<((usize, usize), usize)>> = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    distances[start.0][start.1] = 0;
    queue.push(Reverse((start, 0)));

    while let Some(Reverse((current, _))) = queue.pop() {
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);

        if current == end {
            return Some(reconstruct_path(&distances, start, end));
        }

        for (dx, dy) in &directions {
            let new_x = (current.0 as i32 + dx) as usize;
            let new_y = (current.1 as i32 + dy) as usize;

            if new_x < cosmos.len()
                && new_y < cosmos[0].len()
                && (cosmos[new_x][new_y] == '.'
                    || cosmos[new_x][new_y] == '#'
                    || cosmos[new_x][new_y] == 'S')
            {
                let new_distance = distances[current.0][current.1] + 1;

                if new_distance < distances[new_x][new_y] {
                    distances[new_x][new_y] = new_distance;
                    queue.push(Reverse(((new_x, new_y), new_distance)));
                }
            }
        }
    }
    None
}

// First step is skipped
fn reconstruct_path(
    distances: &Vec<Vec<usize>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut path = Vec::new();
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let (mut x, mut y) = end;
    while (x, y) != start {
        let mut min_distance = usize::MAX;
        let mut next_node = (x, y);

        for (dx, dy) in &directions {
            let new_x = (x as i32 + dx) as usize;
            let new_y = (y as i32 + dy) as usize;
            if new_x < distances.len() && new_y < distances[0].len() {
                let distance = distances[new_x][new_y];
                if distance < min_distance {
                    min_distance = distance;
                    next_node = (new_x, new_y);
                }
            }
        }

        x = next_node.0;
        y = next_node.1;
        path.push((x, y));
    }
    path.reverse();
    path
}

fn find_all_shortest_paths(
    cosmos: &Vec<Vec<char>>,
    galaxies: &Vec<(usize, usize)>,
) -> Vec<(Vec<(usize, usize)>, usize)> {
    let mut shortest_paths = Vec::new();
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            if let Some(path) = shortest_path(cosmos, galaxies[i], galaxies[j]) {
                let cost = path
                    .iter()
                    .map(|&(x, y)| if cosmos[x][y] == 'S' { 100 - 1 } else { 1 })
                    .sum();
                shortest_paths.push((path, cost));
            }
        }
    }
    // for path in shortest_paths.iter() {
    //     // println!("{} -> {}, path.len: {:?}", cosmos[path.0.first().unwrap().0][path.0.first().unwrap().1], cosmos[path.0.last().unwrap().0][path.0.last().unwrap().1], path.0.len());
    //     let y_1 =path.0.first().unwrap().0;
    //     let x_1 =path.0.first().unwrap().1;

    //     let y_2 =path.0.last().unwrap().0;
    //     let x_2 =path.0.last().unwrap().1;

    //     println!("{}:{} -> {}:{}, path.len: {:?}", y_1, x_1, y_2, x_2, path.0.len());
    //     if x_1 == 1 && y_1 == 6 && x_2 == 5 && y_2 == 11 {
    //         println!("path: {:?}", path.0);
    //     }
    // }
    // println!("shortest_paths.len: {:?}", shortest_paths.len());
    shortest_paths
}

fn find_galaxies(cosmos: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    for (i, row) in cosmos.iter().enumerate() {
        for (j, &symbol) in row.iter().enumerate() {
            if symbol == '#' {
                galaxies.push((i, j));
            }
        }
    }
    galaxies
}

fn main() {
    let Ok(lines) = read_lines("input_test.txt") else {
        return;
    };

    let mut cosmos_part_1: Vec<Vec<char>> = Vec::new();
    let mut cosmos_part_2: Vec<Vec<char>> = Vec::new();

    for line in lines {
        match line {
            Ok(line) => {
                let mut cosmic_rows_part_1: Vec<char> = Vec::new();
                let mut cosmic_rows_part_2: Vec<char> = Vec::new();

                for character in line.chars() {
                    cosmic_rows_part_1.push(character);
                    cosmic_rows_part_2.push(character);
                }

                cosmos_part_1.push(cosmic_rows_part_1);
                cosmos_part_2.push(cosmic_rows_part_2);
            }
            _ => panic!("Malformed file"),
        }
    }

    let mut cosmos_1_lenght = cosmos_part_1.len();
    let mut row_index = 0;
    while row_index < cosmos_1_lenght {
        let row = &cosmos_part_1[row_index];
        let mut found_galaxy: bool = false;
        for column_index in 0..row.len() {
            if row[column_index] == '#' {
                found_galaxy = true;
                break;
            }
        }
        if !found_galaxy {
            cosmos_part_1.insert(
                row_index + 1,
                (0..row.len()).map(|_| '.').collect::<Vec<char>>(),
            );
            cosmos_1_lenght += 1;
            row_index += 1;
        }
        row_index += 1;
    }

    let mut column_index: usize = 0;
    let mut columns = cosmos_part_1[0].len();
    while column_index < columns {
        let mut found_galaxy: bool = false;

        for row in cosmos_part_1.iter() {
            if row[column_index] == '#' {
                found_galaxy = true;
                break;
            }
        }

        if !found_galaxy {
            for row in cosmos_part_1.iter_mut() {
                row.insert(column_index, '.');
            }
            columns += 1;
            column_index += 2;
        } else {
            column_index += 1;
        }
    }

    let mut cosmos_2_lenght = cosmos_part_2.len();
    row_index = 0;
    while row_index < cosmos_2_lenght {
        let row = &cosmos_part_2[row_index];
        let mut found_galaxy: bool = false;
        for column_index in 0..row.len() {
            if row[column_index] == '#' {
                found_galaxy = true;
                break;
            }
        }
        if !found_galaxy {
            cosmos_part_2.insert(
                row_index + 1,
                (0..row.len()).map(|_| 'S').collect::<Vec<char>>(),
            );
            cosmos_2_lenght += 1;
            row_index += 1;
        }
        row_index += 1;
    }

    column_index = 0;
    columns = cosmos_part_2[0].len();
    while column_index < columns {
        let mut found_galaxy: bool = false;

        for row in cosmos_part_2.iter() {
            if row[column_index] == '#' {
                found_galaxy = true;
                break;
            }
        }

        if !found_galaxy {
            for row in cosmos_part_2.iter_mut() {
                row.insert(column_index, 'S');
            }
            columns += 1;
            column_index += 2;
        } else {
            column_index += 1;
        }
    }

    // for row in cosmos_part_2.iter() {
    //     for symbol in row.iter() {
    //         print!("{}", symbol);
    //     }
    //     println!();
    // }

    // for row in cosmos_part_1.iter() {
    //     for symbol in row.iter() {
    //         print!("{}", symbol);
    //     }
    //     println!();
    // }

    // for row in cosmos_part_2.iter() {
    //     for symbol in row.iter() {
    //         print!("{}", symbol);
    //     }
    //     println!();
    // }

    let galaxies = find_galaxies(&cosmos_part_1);
    let shortest_paths_part_1 = find_all_shortest_paths(&cosmos_part_1, &galaxies);
    let sum_of_shorted_paths_part_1: usize = shortest_paths_part_1
        .iter()
        .fold(0, |acc, (_, cost)| acc + cost);

    let galaxies_part_2 = find_galaxies(&cosmos_part_2);
    let shortest_paths_part_2 = find_all_shortest_paths(&cosmos_part_2, &galaxies_part_2);
    let sum_of_shorted_paths_part_2: usize = shortest_paths_part_2
        .iter()
        .fold(0, |acc, (_, cost)| acc + cost);

    println!("part 1: {}", sum_of_shorted_paths_part_1);
    println!("part 2: {}", sum_of_shorted_paths_part_2);
}
