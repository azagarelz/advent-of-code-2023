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

fn main() {
    let Ok(lines) = read_lines("input.txt") else {
        return;
    };

    let mut sequences: Vec<Vec<i64>> = Vec::new();

    for line in lines {
        match line {
            Ok(line) => {
                let mut sequence_vec: Vec<i64> = Vec::new();

                line.split_ascii_whitespace().for_each(|sequence_number| {
                    sequence_vec.push(sequence_number.parse::<i64>().unwrap());
                });

                sequences.push(sequence_vec);
            }
            _ => panic!("Malformed file"),
        }
    }

    let mut calculation_sequences: Vec<Vec<i64>> = Vec::new();

    let mut sum_of_new_history_value_part1 = 0;
    let mut sum_of_new_history_value_part2 = 0;

    for sequence in sequences.iter() {
        calculation_sequences.push(sequence.clone());

        loop {
            let sequence_to_iter = calculation_sequences.last().unwrap();

            let mut difference_sequence: Vec<i64> = Vec::new();
            for (index, number) in sequence_to_iter.iter().enumerate() {
                if index != 0 {
                    difference_sequence.push(*number - sequence_to_iter[index - 1]);
                }
            }

            let done = difference_sequence.iter().all(|&x| x == 0);
            calculation_sequences.push(difference_sequence);

            if done {
                let sequences_number = calculation_sequences.len();
                let mut index: i32 = (sequences_number - 1) as i32;

                loop {
                    if index == (sequences_number - 1) as i32 {
                        let calculation_sequence = &mut calculation_sequences[index as usize];
                        calculation_sequence.push(0);
                        calculation_sequence.insert(0, 0);
                    } else {
                        let sequences = &mut calculation_sequences;

                        let current_sequence = &sequences[index as usize];
                        let mut new_history_value_part1 =
                            current_sequence[current_sequence.len() - 1];
                        let mut new_history_value_part2 = current_sequence[0];

                        let mut previous_sequence_index = index + 1;
                        while previous_sequence_index > index {
                            let previous_sequence = &sequences[previous_sequence_index as usize];

                            new_history_value_part1 +=
                                previous_sequence[previous_sequence.len() - 1];
                            new_history_value_part2 -= previous_sequence[0];

                            previous_sequence_index -= 1;
                        }

                        sequences[index as usize].push(new_history_value_part1);
                        sequences[index as usize].insert(0, new_history_value_part2);

                        if index == 0 {
                            sum_of_new_history_value_part1 += new_history_value_part1;
                            sum_of_new_history_value_part2 += new_history_value_part2;
                        }
                    }

                    index -= 1;

                    if index < 0 {
                        break;
                    }
                }

                calculation_sequences.clear();
                break;
            }
        }
    }

    println!("part 1: {}", sum_of_new_history_value_part1);
    println!("part 2: {}", sum_of_new_history_value_part2);
}
