use std::collections::{HashMap, VecDeque};

use num::Integer;
use regex::Regex;

#[derive(Clone)]
enum Type {
    FlipFlop,
    Conjunction,
    Broadcaster,
    Unknown,
}

impl From<&str> for Type {
    fn from(s: &str) -> Self {
        match s {
            "%" => Type::FlipFlop,
            "&" => Type::Conjunction,
            "b" => Type::Broadcaster,
            _ => panic!(),
        }
    }
}

#[derive(Clone)]
struct Module {
    module_type: Type,
    receives_from: HashMap<String, bool>, // last signal received from each input, only remembered for conjunctions
    sends_to: Vec<String>,
    is_on: bool, // only relevant for flip flops
}

struct Pulse {
    from: String,
    to: String,
    is_high: bool,
}

pub fn run_day() {
    let input_str = include_str!("input.txt");
    let module_map = parse_input(input_str);
    println!(
        "Day 20 Part 1: Total high and low pulses multiplied is {}",
        part_one(module_map.clone())
    );
    println!(
        "Day 20 Part 2: Machine turns on after {} presses",
        part_two(module_map)
    );

    println!();
}

fn parse_input(input_str: &str) -> HashMap<String, Module> {
    let name_re = Regex::new(r"[a-z]+").unwrap();
    let mut sent_to_list: Vec<(&str, &str)> = vec![];
    let mut module_map: HashMap<String, Module> = input_str
        .lines()
        .map(|line| {
            let (name, sends_to_list) = line.split_once(" -> ").unwrap();
            let sends_to = name_re
                .captures_iter(sends_to_list)
                .map(|caps| {
                    let to_name = caps.extract::<0>().0;
                    sent_to_list.push((&name[1..], to_name));
                    to_name.to_string()
                })
                .collect();
            let module = Module {
                module_type: name[..1].into(),
                receives_from: HashMap::new(),
                sends_to,
                is_on: false,
            };
            (
                match &name[..1] {
                    "&" | "%" => name[1..].to_owned(),
                    _ => name.to_owned(),
                },
                module,
            )
        })
        .collect();

    for (from_name, to_name) in sent_to_list {
        module_map
            .entry(to_name.to_string())
            .or_insert(Module {
                module_type: Type::Unknown,
                receives_from: HashMap::new(),
                sends_to: vec![],
                is_on: false,
            })
            .receives_from
            .insert(from_name.to_owned(), false);
    }

    module_map
}

fn part_one(mut module_map: HashMap<String, Module>) -> u64 {
    let mut pulses = VecDeque::new();
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        low_pulses += 1; // button sends low pulse

        for name in module_map["broadcaster"].sends_to.iter() {
            pulses.push_back(Pulse {
                from: "broadcaster".to_string(),
                to: name.clone(),
                is_high: false,
            });
            low_pulses += 1; // broadcaster sends low pulses
        }

        while let Some(pulse) = pulses.pop_front() {
            match module_map[&pulse.to].module_type {
                Type::FlipFlop => {
                    if !pulse.is_high {
                        module_map
                            .get_mut(&pulse.to)
                            .expect("No module found!")
                            .is_on = !module_map[&pulse.to].is_on;
                        for to_mod in module_map[&pulse.to].sends_to.iter() {
                            let is_high = module_map[&pulse.to].is_on;
                            pulses.push_back(Pulse {
                                from: pulse.to.to_string(),
                                to: to_mod.to_string(),
                                is_high,
                            });
                            if is_high {
                                high_pulses += 1;
                            } else {
                                low_pulses += 1;
                            }
                        }
                    }
                }
                Type::Conjunction => {
                    *module_map
                        .get_mut(&pulse.to)
                        .expect("No module found!")
                        .receives_from
                        .get_mut(&pulse.from)
                        .expect("No module found!") = pulse.is_high;
                    for to_mod in module_map[&pulse.to].sends_to.iter() {
                        let is_high = !module_map[&pulse.to]
                            .receives_from
                            .values()
                            .all(|is_high| *is_high);
                        pulses.push_back(Pulse {
                            from: pulse.to.to_string(),
                            to: to_mod.to_string(),
                            is_high,
                        });
                        if is_high {
                            high_pulses += 1;
                        } else {
                            low_pulses += 1;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    low_pulses * high_pulses
}

fn part_two(mut module_map: HashMap<String, Module>) -> u128 {
    let mut pulses = VecDeque::new();
    let mut button_press: u128 = 0;

    // assumption (that holds true for my input): rx only receives from one module, and that module is a conjunction
    let mut presses_to_high: HashMap<String, u128> = module_map
        [module_map["rx"].receives_from.keys().next().unwrap()]
    .receives_from
    .keys()
    .map(|key| (key.to_string(), 0))
    .collect();

    // assumptions are also made about the loops so lcm works (like on day 8), which also holds true for my input
    while presses_to_high.values().any(|presses| *presses == 0) {
        button_press += 1;
        for name in module_map["broadcaster"].sends_to.iter() {
            pulses.push_back(Pulse {
                from: "broadcaster".to_string(),
                to: name.clone(),
                is_high: false,
            });
        }

        while let Some(pulse) = pulses.pop_front() {
            if pulse.is_high
                && presses_to_high.contains_key(&pulse.from)
                && presses_to_high[&pulse.from] == 0
            {
                *presses_to_high.get_mut(&pulse.from).unwrap() = button_press;
            }

            match module_map[&pulse.to].module_type {
                Type::FlipFlop => {
                    if !pulse.is_high {
                        module_map
                            .get_mut(&pulse.to)
                            .expect("No module found!")
                            .is_on = !module_map[&pulse.to].is_on;
                        for to_mod in module_map[&pulse.to].sends_to.iter() {
                            let is_high = module_map[&pulse.to].is_on;
                            pulses.push_back(Pulse {
                                from: pulse.to.to_string(),
                                to: to_mod.to_string(),
                                is_high,
                            });
                        }
                    }
                }
                Type::Conjunction => {
                    *module_map
                        .get_mut(&pulse.to)
                        .expect("No module found!")
                        .receives_from
                        .get_mut(&pulse.from)
                        .expect("No module found!") = pulse.is_high;
                    for to_mod in module_map[&pulse.to].sends_to.iter() {
                        let is_high = !module_map[&pulse.to]
                            .receives_from
                            .values()
                            .all(|is_high| *is_high);
                        pulses.push_back(Pulse {
                            from: pulse.to.to_string(),
                            to: to_mod.to_string(),
                            is_high,
                        });
                    }
                }
                _ => {}
            }
        }
    }

    let mut min_presses = 1;
    presses_to_high
        .iter()
        .for_each(|(_, presses)| min_presses = min_presses.lcm(presses));
    min_presses
}
