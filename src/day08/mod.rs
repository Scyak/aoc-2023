use num::integer::lcm;
use std::collections::HashMap;

struct Directions {
    left: String,
    right: String,
}

pub fn run_day() {
    let input = include_str!("input.txt");
    let directions = parse_directions(input);
    let map = parse_map(input);

    println!(
        "Day 8 Part 1: You need to take {} steps.",
        part_one(directions, &map)
    );
    println!(
        "Day 8 Part 2: Ghosts need to take {} steps.",
        part_two(directions, &map)
    );
    println!();
}

fn part_one(directions: &str, map: &HashMap<&str, Directions>) -> u32 {
    let mut steps = 0;
    let mut current_node = "AAA";

    while !current_node.eq("ZZZ") {
        for direction in directions.chars() {
            if direction == 'L' {
                current_node = &map[current_node].left;
            } else {
                current_node = &map[current_node].right;
            }

            steps += 1;

            if current_node.eq("ZZZ") {
                break;
            }
        }
    }

    steps
}

fn part_two(directions: &str, map: &HashMap<&str, Directions>) -> u64 {
    let starting_nodes: Vec<&str> = map
        .keys()
        .filter(|node| node.ends_with("A"))
        .map(|&node| node)
        .collect();
    let mut loop_lengths: Vec<u64> = vec![];

    // figure out length of loop from each starting node to its corresponding end node
    // this only works if there is only one Z node in each loop, which holds true for the input
    // otherwise lcm method would not work
    for starting_node in starting_nodes {
        let mut current_node = starting_node;
        let mut loop_steps = 0;
        while !current_node.ends_with("Z") {
            for direction in directions.chars() {
                if direction == 'L' {
                    current_node = &map[current_node].left;
                } else {
                    current_node = &map[current_node].right;
                }

                loop_steps += 1;

                if current_node.ends_with("Z") {
                    loop_lengths.push(loop_steps);
                    break;
                }
            }
        }
    }

    let mut steps: u64 = 1;

    // lcm of all loop lengths is the number of steps after which the ghost will be at an end node for each path
    for length in loop_lengths {
        steps = lcm(steps, length);
    }

    steps
}

fn parse_directions(input: &str) -> &str {
    let (directions, _) = input.split_once("\n").unwrap();
    directions
}

fn parse_map(input: &str) -> HashMap<&str, Directions> {
    let mut map = HashMap::new();

    // split off directions at empty line
    let (_, map_str) = input.split_once("\n\n").unwrap();

    // extract directions into hashmap (node to Direction with left/right)
    for line in map_str.lines() {
        let (source_node, dir_str) = line.split_once(" = (").unwrap();
        let (left, rest) = dir_str.split_once(", ").unwrap();
        let (right, _) = rest.split_once(")").unwrap();
        let directions = Directions {
            left: left.to_string(),
            right: right.to_string(),
        };
        map.insert(source_node, directions);
    }

    map
}
