use std::collections::HashMap;

pub fn run_day() {
    let input_str = include_str!("input.txt");
    let mut input = parse_input(input_str);
    println!(
        "Day 14 Part 1: Total load after tilting north is {}",
        part_one(&mut input)
    );
    println!(
        "Day 14 Part 2: After 1000000000 spin cycles, load is {}",
        part_two(&input)
    );

    println!();
}

fn parse_input(input_str: &str) -> Vec<Vec<char>> {
    input_str
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn part_one(platform: &mut Vec<Vec<char>>) -> usize {
    // tilt north
    for row_idx in 0..platform.len() {
        for col_idx in 0..platform[row_idx].len() {
            if platform[row_idx][col_idx] == 'O' {
                move_north(platform, row_idx, col_idx);
            }
        }
    }

    calculate_load(platform)
}

fn part_two(input: &Vec<Vec<char>>) -> usize {
    let mut platform = input.to_owned();
    let mut encountered_patterns = HashMap::new();

    let total_cycles = 1000000000;
    let mut cycle = 0;
    let mut loop_found = false;

    while cycle < total_cycles {
        platform = do_spin_cycle(platform.clone());

        if encountered_patterns.contains_key(&platform) && !loop_found {
            // found loop!
            let loop_length = cycle - encountered_patterns[&platform];
            while cycle + loop_length < total_cycles {
                cycle += loop_length;
            }
            loop_found = true;
        } else if !loop_found {
            encountered_patterns.insert(platform.clone(), cycle);
        }

        cycle += 1;
    }

    calculate_load(&platform)
}

fn move_north(platform: &mut Vec<Vec<char>>, row: usize, col: usize) {
    for new_row in (0..row).rev() {
        if platform[new_row][col] != '.' {
            platform[new_row + 1][col] = platform[row][col];

            if (new_row + 1) != row {
                platform[row][col] = '.';
            }
            return;
        }
    }

    // if no replacement was done in the loop, rock must end up in row 0
    platform[0][col] = platform[row][col];
    if 0 != row {
        platform[row][col] = '.';
    }
}

// why implement tilting west, south and east if you can simply rotate the platform clockwise and tilt north again
fn rotate_platform(platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rotated_platform: Vec<Vec<char>> = (0..platform[0].len()).map(|_| vec![]).collect();

    for row in platform.iter().rev() {
        for (col_idx, rock) in row.iter().enumerate() {
            rotated_platform[col_idx].push(*rock);
        }
    }

    rotated_platform
}

fn do_spin_cycle(platform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_platform = platform.to_vec();

    for _ in 0..4 {
        // tilt north
        for row_idx in 0..new_platform.len() {
            for col_idx in 0..new_platform[row_idx].len() {
                if new_platform[row_idx][col_idx] == 'O' {
                    move_north(&mut new_platform, row_idx, col_idx);
                }
            }
        }

        // rotate platform so next face is north
        new_platform = rotate_platform(&new_platform);
    }

    new_platform
}

fn calculate_load(platform: &Vec<Vec<char>>) -> usize {
    platform
        .iter()
        .rev()
        .enumerate()
        .map(|(load, row)| {
            row.iter()
                .map(|rock| match rock {
                    'O' => load + 1,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}
