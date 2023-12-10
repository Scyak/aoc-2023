use std::fs;

#[derive(Debug)]
struct Direction {
    x: i32, // positive: move right, negative: move left
    y: i32, // positive: move down, negative: move up
}

#[derive(Debug)]
struct Pipe {
    shape: char,
    in_loop: bool,
}

pub fn run_day() {
    let input_str = include_str!("input.txt");
    let mut input = parse_input(input_str);
    println!(
        "Day 10 Part 1: Farthest distance is {}",
        part_one(&mut input)
    );
    println!(
        "Day 10 Part 2: There are {} squares inside the loop",
        part_two(&input)
    );
    println!();
}

fn parse_input(input_str: &str) -> Vec<Vec<Pipe>> {
    let input: Vec<Vec<Pipe>> = input_str
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Pipe {
                    shape: c,
                    in_loop: false,
                })
                .collect()
        })
        .collect();
    input
}

fn part_one(input: &mut Vec<Vec<Pipe>>) -> u32 {
    let mut position = (0, 0);
    let mut steps = 0;

    // find starting position
    for (y, line) in input.iter().enumerate() {
        let mut done = false;
        for (x, c) in line.iter().enumerate() {
            if c.shape == 'S' {
                position = (y as i32, x as i32);
                done = true;
                break;
            }
        }
        if done {
            break;
        }
    }

    // find starting direction
    let directions = [
        Direction { x: 1, y: 0 },
        Direction { x: 0, y: 1 },
        Direction { x: -1, y: 0 },
        Direction { x: 0, y: -1 },
    ];
    let mut current_dir = Direction { x: 0, y: 0 };
    for dir in directions {
        let next_y = (position.0 + dir.y) as usize;
        let next_x = (position.1 + dir.x) as usize;
        match get_direction(input[next_y][next_x].shape, &dir) {
            Some(_next_dir) => {
                current_dir = dir;
                position = ((position.0 + current_dir.y), (position.1 + current_dir.x));
                break;
            }
            None => {}
        }
    }

    loop {
        steps += 1;
        let next_y = (position.0) as usize;
        let next_x = (position.1) as usize;
        input[next_y][next_x].in_loop = true;

        current_dir = match get_direction(input[next_y][next_x].shape, &current_dir) {
            Some(dir) => dir,
            None => {
                println!("Something went wrong! Reached dead end. Result will not be correct.");
                break;
            }
        };

        position = ((position.0 + current_dir.y), (position.1 + current_dir.x));

        if input[position.0 as usize][position.1 as usize].shape == 'S' {
            // once found, replace S with the pipe it represents, important for part 2
            let above = &input[(position.0 - 1) as usize][position.1 as usize];
            let below = &input[(position.0 + 1) as usize][position.1 as usize];
            let left = &input[position.0 as usize][(position.1 - 1) as usize];
            let right = &input[position.0 as usize][(position.1 + 1) as usize];

            let loop_above = above.shape == '|' || above.shape == '7' || above.shape == 'F';
            let loop_below = below.shape == '|' || below.shape == 'J' || below.shape == 'L';
            let loop_left = left.shape == '-' || left.shape == 'F' || left.shape == 'L';
            let loop_right = right.shape == '-' || right.shape == '7' || right.shape == 'L';

            let start_shape = match (loop_above, loop_below, loop_left, loop_right) {
                (true, true, false, false) => '|',
                (true, false, true, true) => 'J',
                (true, false, false, true) => 'L',
                (false, true, true, false) => '7',
                (false, true, false, true) => 'F',
                (false, false, true, true) => '-',
                (_, _, _, _) => '?',
            };

            input[position.0 as usize][position.1 as usize].shape = start_shape;
            input[position.0 as usize][position.1 as usize].in_loop = true;

            break;
        }
    }

    (steps / 2) + 1
}

fn part_two(input: &Vec<Vec<Pipe>>) -> u32 {
    let mut inside_squares = 0;
    let mut visual_output: Vec<Vec<char>> = input
        .iter()
        .map(|pipeline| pipeline.iter().map(|pipe| pipe.shape).collect())
        .collect();

    for (r_idx, row) in input.iter().enumerate() {
        let mut inside_loop = false; // at the start of a row, we can never be inside the loop
        for (c_idx, pipe) in row.iter().enumerate() {
            // not loop and loop must be separated by "ascending" pipes
            if pipe.in_loop && (pipe.shape == '|' || pipe.shape == 'J' || pipe.shape == 'L') {
                inside_loop = !inside_loop;
            } else if inside_loop && !pipe.in_loop {
                inside_squares += 1;
            }

            visual_output[r_idx][c_idx] = match (pipe.in_loop, inside_loop) {
                (true, _) => '*',
                (false, true) => 'X',
                (false, false) => ' '
            };
        }
    }

    let visual_str = visual_output.iter().map(|pipeline| (pipeline.into_iter().cloned().collect::<String>() + "\n")).collect::<String>();
    let _ = fs::write("src/day10/output.txt", visual_str);

    inside_squares
}

fn get_direction(pipe: char, incoming_direction: &Direction) -> Option<Direction> {
    match (pipe, incoming_direction.y, incoming_direction.x) {
        ('|', 1, _) => Some(Direction { x: 0, y: 1 }),
        ('|', -1, _) => Some(Direction { x: 0, y: -1 }),
        ('-', _, 1) => Some(Direction { x: 1, y: 0 }),
        ('-', _, -1) => Some(Direction { x: -1, y: 0 }),
        ('L', 1, _) => Some(Direction { x: 1, y: 0 }),
        ('L', _, -1) => Some(Direction { x: 0, y: -1 }),
        ('J', 1, _) => Some(Direction { x: -1, y: 0 }),
        ('J', _, 1) => Some(Direction { x: 0, y: -1 }),
        ('7', -1, _) => Some(Direction { x: -1, y: 0 }),
        ('7', _, 1) => Some(Direction { x: 0, y: 1 }),
        ('F', -1, 0) => Some(Direction { x: 1, y: 0 }),
        ('F', _, -1) => Some(Direction { x: 0, y: 1 }),
        (_, _, _) => None
    }
}
