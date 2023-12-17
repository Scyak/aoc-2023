use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
    None,
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Block {
    row: usize,
    col: usize,
    consecutive_steps: u32,
    last_direction: Direction,
}

pub fn run_day() {
    let input_str = include_str!("input.txt");
    let input = parse_input(input_str);
    println!(
        "Day 17 Part 1: Heat loss on best route is {}",
        part_one_two(&input, false)
    );
    println!(
        "Day 17 Part 2: Heat loss of ultra crucible is {}",
        part_one_two(&input, true)
    );

    println!();
}

fn parse_input(input_str: &str) -> Vec<Vec<u32>> {
    input_str
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part_one_two(city_map: &Vec<Vec<u32>>, part_two: bool) -> u32 {
    let mut min_heat_losses: HashMap<Block, u32> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(u32, Block)>> = BinaryHeap::new();

    let starting_block = Block {
        row: 0,
        col: 0,
        consecutive_steps: 0,
        last_direction: Direction::None,
    };

    min_heat_losses.insert(starting_block, 0);
    heap.push(Reverse((0, starting_block)));

    while let Some(Reverse((route_heat_loss, current_block))) = heap.pop() {
        if current_block.row == (city_map.len() - 1)
            && current_block.col == (city_map[0].len() - 1)
            && (!part_two || current_block.consecutive_steps >= 4)
        {
            return route_heat_loss;
        }

        for adjacent_block in get_adjacent(current_block, &city_map, part_two) {
            let new_route_heat_loss =
                route_heat_loss + city_map[adjacent_block.row][adjacent_block.col];
            if new_route_heat_loss < *min_heat_losses.get(&adjacent_block).unwrap_or(&u32::MAX) {
                heap.push(Reverse((new_route_heat_loss, adjacent_block)));
                min_heat_losses.insert(adjacent_block, new_route_heat_loss);
            }
        }
    }

    0
}

fn get_adjacent(block: Block, city_map: &Vec<Vec<u32>>, part_two: bool) -> Vec<Block> {
    let mut adjacent_blocks = vec![];

    // block above
    if block.row != 0 && block.last_direction != Direction::Down {
        if (!part_two && block.last_direction != Direction::Up)
            || (part_two && block.last_direction != Direction::Up && block.consecutive_steps >= 4)
        {
            adjacent_blocks.push(Block {
                row: block.row - 1,
                col: block.col,
                consecutive_steps: 1,
                last_direction: Direction::Up,
            });
        } else if (!part_two && block.consecutive_steps < 3)
            || (part_two
                && (block.last_direction == Direction::Up
                    || block.last_direction == Direction::None)
                && block.consecutive_steps < 10)
        {
            adjacent_blocks.push(Block {
                row: block.row - 1,
                col: block.col,
                consecutive_steps: block.consecutive_steps + 1,
                last_direction: Direction::Up,
            });
        }
    }

    // block below
    if block.row < (city_map.len() - 1) && block.last_direction != Direction::Up {
        if (!part_two && block.last_direction != Direction::Down)
            || (part_two && block.last_direction != Direction::Down && block.consecutive_steps >= 4)
        {
            adjacent_blocks.push(Block {
                row: block.row + 1,
                col: block.col,
                consecutive_steps: 1,
                last_direction: Direction::Down,
            });
        } else if (!part_two && block.consecutive_steps < 3)
            || (part_two
                && (block.last_direction == Direction::Down
                    || block.last_direction == Direction::None)
                && block.consecutive_steps < 10)
        {
            adjacent_blocks.push(Block {
                row: block.row + 1,
                col: block.col,
                consecutive_steps: block.consecutive_steps + 1,
                last_direction: Direction::Down,
            });
        }
    }

    // block to the left
    if block.col != 0 && block.last_direction != Direction::Right {
        if (!part_two && block.last_direction != Direction::Left)
            || (part_two && block.last_direction != Direction::Left && block.consecutive_steps >= 4)
        {
            adjacent_blocks.push(Block {
                row: block.row,
                col: block.col - 1,
                consecutive_steps: 1,
                last_direction: Direction::Left,
            });
        } else if (!part_two && block.consecutive_steps < 3)
            || (part_two
                && (block.last_direction == Direction::Left
                    || block.last_direction == Direction::None)
                && block.consecutive_steps < 10)
        {
            adjacent_blocks.push(Block {
                row: block.row,
                col: block.col - 1,
                consecutive_steps: block.consecutive_steps + 1,
                last_direction: Direction::Left,
            });
        }
    }

    // block to the right
    if block.col < (city_map[0].len() - 1) && block.last_direction != Direction::Left {
        if (!part_two && block.last_direction != Direction::Right)
            || (part_two
                && block.last_direction != Direction::Right
                && block.consecutive_steps >= 4)
        {
            adjacent_blocks.push(Block {
                row: block.row,
                col: block.col + 1,
                consecutive_steps: 1,
                last_direction: Direction::Right,
            });
        } else if (!part_two && block.consecutive_steps < 3)
            || (part_two
                && (block.last_direction == Direction::Right
                    || block.last_direction == Direction::None)
                && block.consecutive_steps < 10)
        {
            adjacent_blocks.push(Block {
                row: block.row,
                col: block.col + 1,
                consecutive_steps: block.consecutive_steps + 1,
                last_direction: Direction::Right,
            });
        }
    }

    adjacent_blocks
}
