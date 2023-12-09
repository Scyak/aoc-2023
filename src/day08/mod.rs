use std::collections::HashMap;

struct Directions {
    left: String,
    right: String
}

pub fn run_day() {
    let input = include_str!("input.txt");
    let directions = parse_directions(input);
    let map = parse_map(input);

    println!("Day 8 Part 1: You need take {} steps.", part_one(directions, map));
}

fn part_one(directions: &str, map: HashMap<&str, Directions>) -> u32 {
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

fn parse_directions(input: &str) -> &str {
    let (directions, _) = input.split_once("\n").unwrap();
    directions
}

fn parse_map(input: &str) -> HashMap<&str, Directions> {
    let mut map = HashMap::new();

    // split off directions at empty line
    let (_, map_str) = input.split_once("\n\n").unwrap();

    for line in map_str.lines() {
        let (source_node, dir_str) = line.split_once(" = (").unwrap();
        let (left, rest) = dir_str.split_once(", ").unwrap();
        let (right, _) = rest.split_once(")").unwrap();
        let directions = Directions {left: left.to_string(), right: right.to_string()};
        map.insert(source_node, directions);
    }

    map
}