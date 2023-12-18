#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

struct Step {
    direction: Direction,
    distance: i32,
}

pub fn run_day() {
    let input_str = include_str!("input.txt");
    let steps = parse_input(input_str);
    println!(
        "Day 18 Part 1: Lagoon can hold {} cubic meters of lava",
        part_one_two(steps)
    );

    let new_steps = parse_real_input(input_str);
    println!(
        "Day 18 Part 2: Bigger lagoon can hold {} cubic meters of lava",
        part_one_two(new_steps)
    );

    println!()
}

fn parse_input(input_str: &str) -> Vec<Step> {
    let mut steps = vec![];

    for line in input_str.lines() {
        let (dir_str, rest) = line.split_once(" ").unwrap();
        let (dist_str, _) = rest.split_once(" ").unwrap();

        let distance: i32 = dist_str.parse().unwrap();

        let direction = match dir_str {
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "U" => Direction::Up,
            _ => panic!(),
        };

        steps.push(Step {
            direction,
            distance,
        });
    }

    steps
}

fn parse_real_input(input_str: &str) -> Vec<Step> {
    let mut steps = vec![];

    for line in input_str.lines() {
        let (_, input_str) = line.split_once("(#").unwrap();
        let (actual_input_str, _) = input_str.split_once(")").unwrap();

        let distance: i32 = i32::from_str_radix(&actual_input_str[0..5], 16).unwrap();

        let direction = match &actual_input_str[5..] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => panic!(),
        };

        steps.push(Step {
            direction,
            distance,
        });
    }

    steps
}

fn part_one_two(steps: Vec<Step>) -> i64 {
    let mut current_row: i64 = 0;
    let mut current_col: i64 = 0;

    let mut double_area: i64 = 0;
    let mut boundary_points: i64 = 0;

    // shoelace formula to calculate area
    for step in steps {
        let (next_row, next_col) = match step.direction {
            Direction::Right => (current_row, current_col + step.distance as i64),
            Direction::Down => (current_row + step.distance as i64, current_col),
            Direction::Left => (current_row, current_col - step.distance as i64),
            Direction::Up => (current_row - step.distance as i64, current_col),
        };

        double_area += current_col as i64 * next_row as i64;
        double_area -= current_row as i64 * next_col as i64;

        current_row = next_row;
        current_col = next_col;

        boundary_points += step.distance as i64;
    }

    let area = (double_area / 2).abs();

    // solve Pick's theorem for grid points inside
    let grid_points = boundary_points + (area + 1 - (boundary_points / 2));
    grid_points
}
