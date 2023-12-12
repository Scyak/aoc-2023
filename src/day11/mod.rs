use std::vec;

pub fn run_day() {
    let input_str = include_str!("input.txt");
    let input = parse_input(input_str);
    let space_map = expand_space(&input);
    let part_two_map = expand_space_more(&input);

    println!(
        "Day 11 Part 1: Sum of distance between galaxies is {}",
        part_one(&space_map)
    );
    println!(
        "Day 11 Part 2: When expanding more, sum is {}",
        part_two(&part_two_map)
    );

    println!();
}

fn parse_input(input_str: &str) -> Vec<Vec<char>> {
    let input: Vec<Vec<char>> = input_str
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    input
}

fn expand_space(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut space_map = vec![];

    // expand rows
    for row in input.iter() {
        space_map.push(row.to_vec());

        if row.iter().all(|c| *c == '.') {
            // insert row again
            space_map.push(row.to_vec());
        }
    }

    // expand columns
    let mut expanded_columns = 0;
    for col in 0..input[0].len() {
        let mut empty = true;
        for row in input.iter() {
            if row[col] == '#' {
                empty = false;
            }
        }

        if empty {
            for row in space_map.iter_mut() {
                let insert_idx = col + expanded_columns;
                row.insert(insert_idx, '.');
            }
            expanded_columns += 1;
        }
    }

    space_map
}

fn expand_space_more(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut space_map = vec![];

    // expand rows
    for row in input.iter() {
        if row.iter().all(|c| *c == '.') {
            // mark row as expanded vertically
            space_map.push(row.iter().map(|_| '|').collect());
        } else {
            // insert row unchanged
            space_map.push(row.to_vec());
        }
    }

    // expand columns
    for col in 0..space_map[0].len() {
        let mut empty = true;
        for row in space_map.iter() {
            if row[col] == '#' {
                empty = false;
            }
        }

        if empty {
            for row in space_map.iter_mut() {
                if row[col] == '|' {
                    // mark cell as expanded in both directions
                    row[col] = '+';
                } else {
                    // mark cell as expanded horizontally
                    row[col] = '-';
                }
            }
        }
    }

    space_map
}

fn part_one(space_map: &Vec<Vec<char>>) -> usize {
    let mut galaxy_coordinates = vec![];

    for (r_idx, row) in space_map.iter().enumerate() {
        for (c_idx, space) in row.iter().enumerate() {
            if *space == '#' {
                galaxy_coordinates.push((c_idx, r_idx));
            }
        }
    }

    let mut distance_sum = 0;
    for (i, from_galaxy) in galaxy_coordinates.iter().enumerate() {
        for to_galaxy in galaxy_coordinates[(i + 1)..].iter() {
            let x_distance = to_galaxy.0.abs_diff(from_galaxy.0);
            let y_distance = to_galaxy.1.abs_diff(from_galaxy.1);
            distance_sum += x_distance + y_distance;
        }
    }

    distance_sum
}

fn part_two(space_map: &Vec<Vec<char>>) -> usize {
    let mut galaxy_coordinates = vec![];

    for (r_idx, row) in space_map.iter().enumerate() {
        for (c_idx, space) in row.iter().enumerate() {
            if *space == '#' {
                galaxy_coordinates.push((c_idx, r_idx));
            }
        }
    }

    let mut distance_sum = 0;
    for (i, from_galaxy) in galaxy_coordinates.iter().enumerate() {
        for to_galaxy in galaxy_coordinates[(i + 1)..].iter() {
            let from_row = from_galaxy.1.min(to_galaxy.1) as i32;
            let to_row = from_galaxy.1.max(to_galaxy.1) as i32;
            let from_col = from_galaxy.0.min(to_galaxy.0) as i32;
            let to_col = from_galaxy.0.max(to_galaxy.0) as i32;

            for row in (from_row + 1)..(to_row + 1) {
                distance_sum += match space_map[row as usize][from_galaxy.0] {
                    '|' | 'X' => 1000000,
                    _ => 1,
                }
            }

            for col in (from_col + 1)..(to_col + 1) {
                distance_sum += match space_map[from_galaxy.1][col as usize] {
                    '-' | 'X' => 1000000,
                    _ => 1,
                }
            }
        }
    }

    distance_sum
}
