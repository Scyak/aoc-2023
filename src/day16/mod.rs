use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Element {
    Empty,
    MirrorForward,
    MirrorBackward,
    SplitterHorizontal,
    SplitterVertical,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

struct Tile {
    element: Element,
    energized: bool,
}

pub fn run_day() {
    let input_str = include_str!("input.txt");
    let input = parse_input(input_str);
    println!(
        "Day 16 Part 1: There are {} energized tiles",
        part_one(&input)
    );
    println!(
        "Day 16 Part 2: Best configuration yields {} energized tiles",
        part_two(&input)
    );

    println!();
}

fn parse_input(input_str: &str) -> Vec<Vec<Tile>> {
    input_str
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Tile {
                    element: match c {
                        '/' => Element::MirrorForward,
                        '\\' => Element::MirrorBackward,
                        '-' => Element::SplitterHorizontal,
                        '|' => Element::SplitterVertical,
                        _ => Element::Empty,
                    },
                    energized: false,
                })
                .collect()
        })
        .collect()
}

fn part_one(contraption: &Vec<Vec<Tile>>) -> usize {
    get_energized(contraption, 0, 0, Direction::Right)
}

fn part_two(contraption: &Vec<Vec<Tile>>) -> usize {
    let mut energized_tiles = 0;

    for starting_row in 0..contraption.len() {
        energized_tiles = energized_tiles.max(get_energized(
            contraption,
            starting_row,
            0,
            Direction::Right,
        ));
        energized_tiles = energized_tiles.max(get_energized(
            contraption,
            starting_row,
            contraption[starting_row].len() - 1,
            Direction::Left,
        ));
    }

    for starting_col in 0..contraption[0].len() {
        energized_tiles = energized_tiles.max(get_energized(
            contraption,
            0,
            starting_col,
            Direction::Down,
        ));
        energized_tiles = energized_tiles.max(get_energized(
            contraption,
            contraption.len() - 1,
            starting_col,
            Direction::Up,
        ));
    }

    energized_tiles
}

fn get_energized(
    input_contraption: &Vec<Vec<Tile>>,
    starting_row: usize,
    starting_col: usize,
    starting_dir: Direction,
) -> usize {
    let mut contraption: Vec<Vec<Tile>> = input_contraption
        .iter()
        .map(|line| {
            line.iter()
                .map(|tile| Tile {
                    element: tile.element,
                    energized: false,
                })
                .collect()
        })
        .collect();

    let mut done_steps: HashSet<(usize, usize, Direction)> = HashSet::new();
    trace_beam(
        starting_row,
        starting_col,
        starting_dir,
        &mut contraption,
        &mut done_steps,
    );

    contraption
        .iter()
        .map(|row| row.iter().filter(|tile| tile.energized).count())
        .sum()
}

fn trace_beam(
    starting_row: usize,
    starting_col: usize,
    starting_dir: Direction,
    contraption: &mut Vec<Vec<Tile>>,
    done_steps: &mut HashSet<(usize, usize, Direction)>,
) {
    let mut row = starting_row;
    let mut col = starting_col;
    let mut direction = starting_dir;
    let mut exited_contraption = false;

    while !exited_contraption && row < contraption.len() && col < contraption[row].len() {
        if done_steps.contains(&(row, col, direction)) {
            // step has already been done, result will not change, do not repeat
            return;
        }

        done_steps.insert((row, col, direction));
        contraption[row][col].energized = true;

        match contraption[row][col].element {
            Element::Empty => {
                (row, col, exited_contraption) = step_in_direction(row, col, direction)
            }
            Element::MirrorForward => {
                direction = match direction {
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Right,
                };
                (row, col, exited_contraption) = step_in_direction(row, col, direction);
            }
            Element::MirrorBackward => {
                direction = match direction {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Left,
                };
                (row, col, exited_contraption) = step_in_direction(row, col, direction);
            }
            Element::SplitterHorizontal => match direction {
                Direction::Left | Direction::Right => {
                    (row, col, exited_contraption) = step_in_direction(row, col, direction)
                }
                Direction::Down | Direction::Up => {
                    trace_beam(row, col, Direction::Right, contraption, done_steps);
                    trace_beam(row, col, Direction::Left, contraption, done_steps);
                }
            },
            Element::SplitterVertical => match direction {
                Direction::Down | Direction::Up => {
                    (row, col, exited_contraption) = step_in_direction(row, col, direction)
                }
                Direction::Right | Direction::Left => {
                    trace_beam(row, col, Direction::Up, contraption, done_steps);
                    trace_beam(row, col, Direction::Down, contraption, done_steps);
                }
            },
        }
    }
}

/// takes row, column and direction and returns new position (row, col) after one step as well as whether the value would have become negative
fn step_in_direction(row: usize, col: usize, direction: Direction) -> (usize, usize, bool) {
    match direction {
        Direction::Right => (row, col + 1, false),
        Direction::Down => (row + 1, col, false),
        Direction::Left => match col {
            0 => (row, 0, true),
            _ => (row, col - 1, false),
        },
        Direction::Up => match row {
            0 => (0, col, true),
            _ => (row - 1, col, false),
        },
    }
}
