use regex::Regex;
use regex::RegexSet;
use std::fs::read_to_string;

pub fn run_day() {
    println!(
        "Day 2 Part 1: The sum of possible game ids is {}",
        part_one()
    );
    println!(
        "Day 2 Part 2: The sum of game powers is {}",
        part_two()
    );
    println!();
}

pub fn part_one() -> usize {
    // regexes to check for {>12} red, {>13} green, {>14} blue
    let regex_set = RegexSet::new(&[
        r"(1[3-9]|[2-9]\d|\d{3,}) red",
        r"(1[4-9]|[2-9]\d|\d{3,}) green",
        r"(1[5-9]|[2-9]\d|\d{3,}) blue",
    ])
    .unwrap();

    let mut game_sum = 0;

    for (line_nr, line) in read_to_string("src/day02/input.txt")
        .unwrap()
        .lines()
        .enumerate()
    {
        if !regex_set.is_match(line) {
            game_sum += line_nr + 1; //game nr is one more than line number
        }
    }

    game_sum
}

pub fn part_two() -> usize {
    // regexes with groups to extract amounts
    let red_regex = Regex::new(r"(\d+) red").unwrap();
    let green_regex = Regex::new(r"(\d+) green").unwrap();
    let blue_regex = Regex::new(r"(\d+) blue").unwrap();

    let mut power_sum = 0;

    for line in read_to_string("src/day02/input.txt")
        .unwrap()
        .lines()
    {
        // find biggest amount of red cubes
        let mut max_red: usize = 0;
        for(_, [num_str]) in red_regex.captures_iter(line).map(|caps| caps.extract()) {
            let num: usize = num_str.parse().unwrap();
            if num > max_red {
                max_red = num;
            }
        }
        
        // find biggest amount of green cubes
        let mut max_green: usize = 0;
        for(_, [num_str]) in green_regex.captures_iter(line).map(|caps| caps.extract()) {
            let num: usize = num_str.parse().unwrap();
            if num > max_green {
                max_green = num;
            }
        }

        // find biggest amount of blue cubes
        let mut max_blue: usize = 0;
        for(_, [num_str]) in blue_regex.captures_iter(line).map(|caps| caps.extract()) {
            let num: usize = num_str.parse().unwrap();
            if num > max_blue {
                max_blue = num;
            }
        }

        power_sum += max_red * max_green * max_blue;
    }

    power_sum
}
