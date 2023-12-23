use std::collections::{HashMap, HashSet, VecDeque};

const NEIGHBORS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Clone)]
struct Step {
    coordinates: (usize, usize),
    step_nr: usize,
    visited: HashSet<(usize, usize)>,
}

pub fn run_day() {
    let input_str = include_str!("input.txt");
    let map = parse_input(input_str);
    println!("Day 23 Part 1: Longest hike has {} steps", part_one(&map));
    println!(
        "Day 23 Part 2: Longest hike with scramling up hills has {} steps",
        part_two(&map)
    );
}

fn parse_input(input_str: &str) -> Vec<Vec<char>> {
    input_str
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn part_one(map: &Vec<Vec<char>>) -> usize {
    let mut hike_lengths: Vec<usize> = Vec::new();
    let mut steps: VecDeque<Step> = VecDeque::new();

    let map_rows = map.len();
    let map_cols = map[0].len();

    steps.push_back(Step {
        coordinates: (0, 1),
        step_nr: 0,
        visited: [(0, 1)].into(),
    });

    while let Some(step) = steps.pop_front() {
        if step.coordinates.0 == map_rows - 1 && step.coordinates.1 == map_cols - 2 {
            // reached the end of the hike
            hike_lengths.push(step.step_nr);
            continue;
        }

        match map[step.coordinates.0][step.coordinates.1] {
            '#' => {}
            '>' => steps.append(&mut step_in_directions(&step, [(0, 1)].into(), &map)),
            'v' => steps.append(&mut step_in_directions(&step, [(1, 0)].into(), &map)),
            '.' => steps.append(&mut step_in_directions(&step, NEIGHBORS.into(), &map)),
            _ => panic!("Unexpected char!"),
        }
    }

    *hike_lengths.iter().max().unwrap()
}

fn part_two(map: &Vec<Vec<char>>) -> usize {
    let start = (0, 1);
    let end = (map.len() - 1, map[0].len() - 2);
    let mut nodes: Vec<(usize, usize)> = [start, end].into();

    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] != '#' && map[r][c] != '>' && map[r][c] != 'v' {
                let step = Step {
                    coordinates: (r, c),
                    step_nr: 0,
                    visited: HashSet::new(),
                };
                match step_in_directions(&step, NEIGHBORS.into(), map).len() {
                    3 | 4 => nodes.push((r, c)),
                    _ => {}
                }
            }
        }
    }

    let mut adjacency_map: HashMap<(usize, usize), HashMap<(usize, usize), usize>> = HashMap::new();

    for &node in nodes.iter() {
        let mut steps: VecDeque<Step> = VecDeque::new();

        steps.push_back(Step {
            coordinates: node,
            step_nr: 0,
            visited: [node].into(),
        });

        while let Some(step) = steps.pop_front() {
            if nodes.contains(&step.coordinates) && step.coordinates != node {
                // found next graph node
                adjacency_map
                    .entry(node)
                    .or_default()
                    .entry(step.coordinates)
                    .or_insert(step.step_nr);
                continue;
            }

            match map[step.coordinates.0][step.coordinates.1] {
                '#' => {}
                '.' | '>' | 'v' => {
                    steps.append(&mut step_in_directions(&step, NEIGHBORS.into(), &map))
                }
                _ => panic!("Unexpected char!"),
            }
        }
    }

    longest_distance(start, end, &adjacency_map, HashSet::new())
}

fn step_in_directions(
    step: &Step,
    directions: Vec<(isize, isize)>,
    map: &Vec<Vec<char>>,
) -> VecDeque<Step> {
    let mut next_steps = VecDeque::new();

    for direction in directions {
        if (step.coordinates.0 == 0 && direction.0 < 0)
            || (step.coordinates.1 == 0 && direction.1 < 0)
        {
            continue;
        }

        let next_coordinates = (
            (step.coordinates.0 as isize + direction.0) as usize,
            (step.coordinates.1 as isize + direction.1) as usize,
        );

        if next_coordinates.0 >= map.len()
            || next_coordinates.1 >= map[0].len()
            || step.visited.contains(&next_coordinates)
            || map[next_coordinates.0][next_coordinates.1] == '#'
        {
            continue;
        }

        let mut next_step = Step {
            coordinates: next_coordinates,
            step_nr: step.step_nr + 1,
            visited: step.visited.clone(),
        };

        next_step.visited.insert(step.coordinates);
        next_steps.push_back(next_step);
    }

    next_steps
}

fn longest_distance(
    start: (usize, usize),
    end: (usize, usize),
    adjacency_map: &HashMap<(usize, usize), HashMap<(usize, usize), usize>>,
    visited: HashSet<(usize, usize)>,
) -> usize {
    if start == end {
        return 0;
    }

    let mut max_distance = 0;

    for (&adjacent_node, &distance) in adjacency_map[&start].iter() {
        if visited.contains(&adjacent_node) {
            continue;
        }

        let mut new_visited = visited.clone();
        new_visited.insert(adjacent_node);

        max_distance = max_distance
            .max(distance + longest_distance(adjacent_node, end, adjacency_map, new_visited));
    }

    max_distance
}
