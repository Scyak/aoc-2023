use std::fs::read_to_string;

const DIGITS_TO_FIND: [(&str, u32); 18] = [
    ("1", 1),
    ("one", 1),
    ("2", 2),
    ("two", 2),
    ("3", 3),
    ("three", 3),
    ("4", 4),
    ("four", 4),
    ("5", 5),
    ("five", 5),
    ("6", 6),
    ("six", 6),
    ("7", 7),
    ("seven", 7),
    ("8", 8),
    ("eight", 8),
    ("9", 9),
    ("nine", 9),
];

pub fn run_day() {
    println!("Day 1 Part 1: The sum of the calibration values (digits only) is {}", part_one());
    println!("Day 1 Part 2: The actual sum including words is {}", part_two());
    println!();
}

fn part_one() -> u32 {
    let mut sum = 0;

    for line in read_to_string("src/day01/input.txt").unwrap().lines() {
        let mut first = true;
        let mut first_digit = 0;
        let mut last_digit = 0;

        for char in line.chars() {
            if char.is_digit(10) {
                let digit = char.to_digit(10).expect("Not a digit!");
                if first {
                    first_digit = digit;
                    last_digit = digit;
                    first = false;
                } else {
                    last_digit = digit;
                }
            }
        }

        sum = sum + (10 * first_digit) + last_digit;
    }

    sum
}

fn part_two() -> u32 {
    let mut sum = 0;

    for line in read_to_string("src/day01/input.txt").unwrap().lines() {
        let mut first_digit = 0;
        let mut first_digit_idx = usize::MAX;
        let mut last_digit = 0;
        let mut last_digit_idx: usize = 0;

        for digit in DIGITS_TO_FIND {
            // find first digit
            match line.find(digit.0) {
                None => (),
                Some(idx) => {
                    if idx <= first_digit_idx {
                        first_digit = digit.1;
                        first_digit_idx = idx;
                    }
                }
            }

            // find last digit
            match line.rfind(digit.0) {
                None => (),
                Some(idx) => {
                    if idx >= last_digit_idx {
                        last_digit = digit.1;
                        last_digit_idx = idx;
                    }
                }
            }
        }

        sum = sum + (10 * first_digit) + last_digit;
    }

    sum
}