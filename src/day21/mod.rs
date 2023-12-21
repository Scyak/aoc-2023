use std::collections::{HashSet, VecDeque};

struct Step {
    row: isize,
    col: isize,
    step_nr: u32,
}

pub fn run_day() {
    let input_str = include_str!("input.txt");
    let (passable, (starting_row, starting_col)) = parse_input(input_str);

    println!(
        "Day 21 Part 1: Can reach {} plots in 64 steps",
        part_one(&passable, starting_row, starting_col, 64)
    );
    println!(
        "Day 21 Part 2: Can reach {} plots in 26501365 steps",
        part_two(&passable, starting_row, starting_col, 26501365)
    );

    println!();
}

/// returns bool map of plots (true: passable, false: rock) and tuple of starting coordinates (row, col)
fn parse_input(input_str: &str) -> (Vec<Vec<bool>>, (usize, usize)) {
    let mut starting_row = 0;
    let mut starting_col = 0;

    let map = input_str
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => true,
                    '#' => false,
                    'S' => {
                        starting_row = row;
                        starting_col = col;
                        true
                    }
                    _ => panic!("Unexpected input char!"),
                })
                .collect()
        })
        .collect();

    (map, (starting_row, starting_col))
}

fn part_one(
    passable: &Vec<Vec<bool>>,
    starting_row: usize,
    starting_col: usize,
    step_count: u32,
) -> u32 {
    let mut reachable_plots = 0;
    let mut steps: VecDeque<Step> = VecDeque::new();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    let step_mod = step_count % 2; // anything that's reachable in an even amount of steps is also reachable in a larger even amount by going back and forth, same for uneven

    steps.push_back(Step {
        row: starting_row as isize,
        col: starting_col as isize,
        step_nr: 0,
    });

    while let Some(step) = steps.pop_front() {
        let map_row = step.row.rem_euclid(passable.len() as isize) as usize;
        let map_col = step.col.rem_euclid(passable[0].len() as isize) as usize;

        if visited.contains(&(step.row, step.col))
            || step.step_nr > step_count
            || !passable[map_row][map_col]
        {
            continue;
        }

        visited.insert((step.row, step.col));

        if step.step_nr % 2 == step_mod {
            reachable_plots += 1;
        }

        // take steps in all four directions
        steps.push_back(Step {
            row: step.row + 1,
            col: step.col,
            step_nr: step.step_nr + 1,
        });
        steps.push_back(Step {
            row: step.row - 1,
            col: step.col,
            step_nr: step.step_nr + 1,
        });
        steps.push_back(Step {
            row: step.row,
            col: step.col + 1,
            step_nr: step.step_nr + 1,
        });
        steps.push_back(Step {
            row: step.row,
            col: step.col - 1,
            step_nr: step.step_nr + 1,
        });
    }

    reachable_plots
}

fn part_two(
    passable: &Vec<Vec<bool>>,
    starting_row: usize,
    starting_col: usize,
    step_count: u32,
) -> usize {
    // assumption: step count is n*(input side) + (steps to reach input edge)
    // I genuinely have no idea how this works, I just looked at some data points and found the quadratic progression

    // the following code just interpolates a quadratic function from three points (with n=0, n=1 and n=2)
    let steps_to_edge = passable.len() as u32 / 2;
    let c = part_one(passable, starting_row, starting_col, steps_to_edge as u32) as f64;

    let x1: f64 = 1.0;
    let y1 = part_one(
        passable,
        starting_row,
        starting_col,
        x1 as u32 * passable.len() as u32 + steps_to_edge,
    ) as f64;

    let x2: f64 = 2.0;
    let y2 = part_one(
        passable,
        starting_row,
        starting_col,
        x2 as u32 * passable.len() as u32 + steps_to_edge,
    ) as f64;

    let a = y1 / ((x1 - x2) * x1) + y2 / ((x2 - x1) * x2) + c / (x1 * x2);

    let b = 0.0
        - y1 * x2 / ((x1 - x2) * x1)
        - y2 * x1 / ((x2 - x1) * x2)
        - c * (x1 + x2) / (x1 * x2);

    let n: usize = (step_count as usize - 65) / 131;

    a as usize * n * n + b as usize * n + c as usize
}
