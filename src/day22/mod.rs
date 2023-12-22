use std::collections::HashMap;

use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Coordinate {
    start: usize,
    end: usize,
}

impl Coordinate {
    fn overlaps(&self, other: &Coordinate) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

#[derive(Debug)]
struct Brick {
    x: Coordinate,
    y: Coordinate,
    z: Coordinate,
    rests_on: Vec<usize>,
}

impl Brick {
    fn overlaps(&self, other: &Brick) -> bool {
        self.x.overlaps(&other.x) && self.y.overlaps(&other.y)
    }

    fn rests_on(&self, other: &Brick) -> bool {
        if self.z.start == 0 {
            return false;
        }

        self.overlaps(other) && other.z.end == self.z.start - 1
    }
}

pub fn run_day() {
    let input_str = include_str!("input.txt");
    let bricks = parse_input(input_str);
    println!(
        "Day 22 Part 1: Could disintegrate {} bricks individually",
        part_one(&bricks)
    );
    println!(
        "Day 22 Part 2: Disintegrating each brick individually, {} bricks would fall",
        part_two(&bricks)
    );
}

fn parse_input(input_str: &str) -> HashMap<usize, Brick> {
    let re = Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap();
    let mut bricks: Vec<Brick> = re
        .captures_iter(input_str)
        .map(|caps| {
            let (_, [x_from, y_from, z_from, x_to, y_to, z_to]) = caps.extract();
            Brick {
                x: Coordinate {
                    start: x_from.parse().unwrap(),
                    end: x_to.parse().unwrap(),
                },
                y: Coordinate {
                    start: y_from.parse().unwrap(),
                    end: y_to.parse().unwrap(),
                },
                z: Coordinate {
                    start: z_from.parse().unwrap(),
                    end: z_to.parse().unwrap(),
                },
                rests_on: Vec::new(),
            }
        })
        .collect();

    bricks.sort_by(|a: &Brick, b| a.z.start.cmp(&b.z.start));

    let mut dropped_bricks: HashMap<usize, Brick> = HashMap::new();

    for (idx, brick) in bricks.into_iter().enumerate() {
        dropped_bricks.insert(idx, drop_brick(&dropped_bricks, brick));
    }

    dropped_bricks
}

fn part_one(bricks: &HashMap<usize, Brick>) -> usize {
    let mut can_disintegrate: Vec<usize> = bricks.keys().map(|&key| key).collect();

    for brick in bricks.values() {
        if brick.rests_on.len() == 1 {
            can_disintegrate.retain(|&brick_id| brick_id != brick.rests_on[0]);
        }
    }

    can_disintegrate.len()
}

fn part_two(bricks: &HashMap<usize, Brick>) -> usize {
    let mut dropping_sum = 0;

    for &brick_id in bricks.keys() {
        let mut dropping: Vec<usize> = bricks
            .iter()
            .filter(|(_, brick)| brick.rests_on.len() == 1 && brick.rests_on[0] == brick_id)
            .map(|(&id, _)| id)
            .collect();

        loop {
            let mut newly_dropping: Vec<usize> = bricks
                .iter()
                .filter(|(id, brick)| {
                    !dropping.contains(id)
                        && !brick.rests_on.is_empty()
                        && brick.rests_on.iter().all(|id| dropping.contains(id))
                })
                .map(|(&id, _)| id)
                .collect();

            if newly_dropping.is_empty() {
                break;
            }

            dropping.append(&mut newly_dropping);
        }

        dropping_sum += dropping.len();
    }

    dropping_sum
}

fn drop_brick(resting_bricks: &HashMap<usize, Brick>, brick: Brick) -> Brick {
    let mut new_z = 0;
    resting_bricks
        .values()
        .filter(|resting| resting.overlaps(&brick))
        .for_each(|resting| new_z = new_z.max(resting.z.end + 1));

    let mut brick = Brick {
        x: brick.x,
        y: brick.y,
        z: Coordinate {
            start: new_z,
            end: new_z + brick.z.end - brick.z.start,
        },
        rests_on: Vec::new(),
    };
    brick.rests_on = resting_bricks
        .iter()
        .filter(|(_, other_brick)| brick.rests_on(other_brick))
        .map(|(id, _)| *id)
        .collect();
    brick
}
