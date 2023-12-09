use std::fs::read_to_string;

pub fn run_day() {
    let input = to_char_vec(read_to_string("src/day03/input.txt").expect("File could not be opened"));
    println!("Day 3 Part 1: The sum of the calibration numbers is {}", part_one(&input));
}


fn to_char_vec(input: String) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.chars().collect());
    }
    result
}

fn check_adjacency(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    for i in -1..2 {
        let y_idx_signed = y as i32 + i;
        let mut y_idx = 0;
        if y_idx_signed > 0 {
            y_idx = y_idx_signed as usize;
        }
        for j in -1..2 {
            let x_idx_signed = x as i32 + j;
            let mut x_idx = 0;
            if x_idx_signed > 0 {
                x_idx = x_idx_signed as usize;
            }
            match input.get(y_idx) {
                Some(vec) => {
                    match vec.get(x_idx) {
                        Some(c) => {
                            if *c != '.' && !c.is_digit(10) {
                                return true;
                            }
                        },
                        None => {}
                    }
                },
                None => {}
            }
            }
        }
    false
}

fn part_one(input: &Vec<Vec<char>>) -> u32 {
    let mut sum = 0;

    let mut current_number = 0;
    let mut current_adjacency = false;
    for (y, line) in input.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char.is_digit(10) {
                current_number = current_number * 10 + char.to_digit(10).expect("Not a digit");
                if check_adjacency(input, x, y) {
                    current_adjacency = true;
                }
            } else {
                if current_adjacency {
                    println!("Found number: {}", current_number);
                    sum += current_number;
                    current_adjacency = false;
                }
                current_number = 0;
            }
        }
        if current_adjacency {
            println!("Found number: {}", current_number);
            sum += current_number;
            current_adjacency = false;
        }
        current_number = 0;
    }

    sum
}