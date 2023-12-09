pub fn run_day() {
    let input_str = include_str!("input.txt");
    let input = parse_input(input_str);
    println!(
        "Day 9 Part 1: Sum of extrapolated values continuing series is {}",
        part_one(&input)
    );
    println!(
        "Day 9 Part 1: Sum of extrapolated values before series is {}",
        part_two(&input)
    );
    println!();
}

fn parse_input(input_str: &str) -> Vec<Vec<i32>> {
    let input = input_str
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();
    input
}

fn part_one(input: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for series in input {
        sum += get_next_value(series);
    }

    sum
}

fn get_next_value(series: &Vec<i32>) -> i32 {
    // if all values are the same (0 differences) next value is obvious
    if series.iter().min() == series.iter().max() {
        return series[0];
    }

    let differences: Vec<i32> = series.windows(2).into_iter().map(|w| w[1] - w[0]).collect();

    return series.last().unwrap() + get_next_value(&differences);
}

fn part_two(input: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for series in input {
        sum += get_previous_value(series);
    }

    sum
}

fn get_previous_value(series: &Vec<i32>) -> i32 {
    // if all values are the same (0 differences) previous value is obvious
    if series.iter().min() == series.iter().max() {
        return series[0];
    }

    let differences: Vec<i32> = series.windows(2).into_iter().map(|w| w[1] - w[0]).collect();

    return series.first().unwrap() - get_previous_value(&differences);
}
