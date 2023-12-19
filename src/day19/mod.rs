use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Attribute {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

impl From<&str> for Attribute {
    fn from(s: &str) -> Self {
        match s {
            "x" => Attribute::X,
            "m" => Attribute::M,
            "a" => Attribute::A,
            "s" => Attribute::S,
            _ => panic!(),
        }
    }
}

struct Condition {
    attribute: Attribute,
    greater: bool,
    target: u64,
    result: String,
}

pub fn run_day() {
    let input_str = include_str!("input.txt");
    let (workflows, parts) = parse_input(input_str);
    println!(
        "Day 19 Part 1: The sum of accepted part attributes is {}",
        part_one(&workflows, parts)
    );
    println!(
        "Day 19 Part 2: The sum of possible parts that would be accepted is {}",
        part_two(&workflows)
    );

    println!()
}

fn parse_input(input_str: &str) -> (HashMap<String, (Vec<Condition>, String)>, Vec<Vec<u64>>) {
    let (workflow_str, part_str) = input_str.split_once("\n\n").unwrap();

    let workflow_re = Regex::new(r"(x|m|a|s)(<|>)([0-9]+):([A-Za-z]+)").unwrap();
    let default_re = Regex::new(r",([A-Za-z]+)}").unwrap();
    let workflows = workflow_str
        .lines()
        .map(|line| {
            let (name, condition_str) = line.split_once("{").unwrap();
            let conditions = workflow_re
                .captures_iter(condition_str)
                .map(|caps| {
                    let (_, [attr, op, target, result]) = caps.extract();
                    Condition {
                        attribute: attr.into(),
                        greater: (op == ">"),
                        target: target.parse().unwrap(),
                        result: result.to_string(),
                    }
                })
                .collect();
            let default = default_re.captures(condition_str).unwrap()[1].to_owned();
            (name.to_string(), (conditions, default))
        })
        .collect();

    let parts_re = Regex::new(r"\d+").unwrap();
    let parts = part_str
        .lines()
        .map(|line| {
            parts_re
                .captures_iter(line)
                .map(|caps| caps[0].parse().unwrap())
                .collect()
        })
        .collect();

    (workflows, parts)
}

fn part_one(workflows: &HashMap<String, (Vec<Condition>, String)>, parts: Vec<Vec<u64>>) -> u64 {
    parts
        .into_iter()
        .filter(|part| is_accepted(part, &workflows, "in"))
        .map(|part| part.iter().sum::<u64>())
        .sum()
}

fn is_accepted(
    part: &Vec<u64>,
    workflows: &HashMap<String, (Vec<Condition>, String)>,
    workflow: &str,
) -> bool {
    let conditions = &workflows[workflow].0;

    for condition in conditions {
        // != is logical xor
        // xor-ing the lesser condition and whether the condition should be greater will check condition for either operator
        if (part[condition.attribute as usize] < condition.target) != condition.greater {
            match &condition.result[..] {
                "R" => return false,
                "A" => return true,
                workflow => return is_accepted(part, workflows, workflow),
            };
        }
    }

    match &workflows[workflow].1[..] {
        "R" => return false,
        "A" => return true,
        workflow => return is_accepted(part, workflows, workflow),
    };
}

fn part_two(workflows: &HashMap<String, (Vec<Condition>, String)>) -> u64 {
    let part_ranges: Vec<(u64, u64)> = (0..4).map(|_| (1, 4000)).collect();

    accept_ranges(&part_ranges, workflows, "in")
}

fn accept_ranges(
    part_ranges: &Vec<(u64, u64)>,
    workflows: &HashMap<String, (Vec<Condition>, String)>,
    workflow: &str,
) -> u64 {
    let conditions = &workflows[workflow].0;
    let mut ranges = part_ranges.clone();
    let mut sum: u64 = 0;
    let mut done = false;

    for condition in conditions {
        // check if any part of the range fulfills the condition
        if condition.greater && ranges[condition.attribute as usize].1 > condition.target {
            // if everything is greater, just throw entire part ranges into new workflow
            if ranges[condition.attribute as usize].0 > condition.target {
                sum += match &condition.result[..] {
                    "R" => 0,
                    "A" => ranges
                        .iter()
                        .map(|(min, max)| max - min + 1)
                        .product::<u64>(),
                    workflow => accept_ranges(&ranges, workflows, workflow),
                };
                done = true;
                break;
            }

             // if not everything is greater, split the ranges
            // keep going for the leftover parts of this one, and check the new one with its new workflow
            let mut new_ranges = ranges.clone();
            new_ranges[condition.attribute as usize].0 = new_ranges[condition.attribute as usize]
                .0
                .max(condition.target + 1);
            ranges[condition.attribute as usize].1 = condition.target;
            sum += match &condition.result[..] {
                "R" => 0,
                "A" => new_ranges
                    .iter()
                    .map(|(min, max)| max - min + 1)
                    .product::<u64>(),
                workflow => accept_ranges(&new_ranges, workflows, workflow),
            };
        } else if !condition.greater && ranges[condition.attribute as usize].0 < condition.target {
            // if everything is smaller, just throw entire part ranges into new workflow
            if ranges[condition.attribute as usize].1 < condition.target {
                sum += match &condition.result[..] {
                    "R" => 0,
                    "A" => ranges
                        .iter()
                        .map(|(min, max)| max - min + 1)
                        .product::<u64>(),
                    workflow => accept_ranges(&ranges, workflows, workflow),
                };
                done = true;
                break;
            }

            // if not everything is smaller, split the ranges
            // keep going for the leftover parts of this one, and check the new one with its new workflow
            let mut new_ranges = ranges.clone();
            new_ranges[condition.attribute as usize].1 = new_ranges[condition.attribute as usize]
                .1
                .min(condition.target - 1);
            ranges[condition.attribute as usize].0 = condition.target;
            sum += match &condition.result[..] {
                "R" => 0,
                "A" => new_ranges
                    .iter()
                    .map(|(min, max)| max - min + 1)
                    .product::<u64>(),
                workflow => accept_ranges(&new_ranges, workflows, workflow),
            };
        }
    }

    if !done {
        sum += match &workflows[workflow].1[..] {
            "R" => 0,
            "A" => ranges
                .iter()
                .map(|(min, max)| max - min + 1)
                .product::<u64>(),
            workflow => accept_ranges(&ranges, workflows, workflow),
        };
    }

    sum
}
