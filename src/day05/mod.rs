pub fn run_day() {
    println!("Day 5 Part 1: The lowest location nr is {}", part_one());
    println!(
        "Day 5 Part 2: The actual lowest location nr is {}",
        part_two()
    );
    println!();
}

struct Seed {
    number: u32,
    converted: bool,
}

#[derive(Debug)]
struct SeedRange {
    start: u64,
    end: u64, // exclusive
    converted: bool,
}

fn part_one() -> u32 {
    let input = include_str!("input.txt");
    let (seed_str, maps) = input.split_once("\n\n").unwrap(); // split once by empty line

    // extract seeds
    let (_, seed_nrs) = seed_str.split_once(": ").unwrap();
    let seed_nr_strs: Vec<&str> = seed_nrs.split_whitespace().collect();
    let mut seeds = vec![];
    for seed_num in seed_nr_strs {
        let seed = Seed {
            number: seed_num.parse().expect("Not a number"),
            converted: false,
        };
        seeds.push(seed);
    }

    // split into conversion blocks by empty lines
    let blocks = maps.split("\n\n");
    for block in blocks {
        let (_, ranges) = block.split_once("\n").unwrap(); // toss header
        for range in ranges.lines() {
            let (dest_str, back_part) = range.split_once(" ").unwrap();
            let (source_str, range_length_str) = back_part.split_once(" ").unwrap();
            let dest_start: u32 = dest_str.parse().expect("Not a number");
            let source_start: u32 = source_str.parse().expect("Not a number");
            let range_length: u32 = range_length_str.parse().expect("Not a number");

            for seed in &mut seeds {
                if seed.converted || seed.number < source_start {
                    continue;
                }

                let from_range_start = seed.number - source_start;
                if from_range_start < range_length {
                    seed.number = dest_start + from_range_start;
                    seed.converted = true;
                }
            }
        }

        // reset conversion markers
        for seed in &mut seeds {
            seed.converted = false;
        }
    }

    let min_num = seeds
        .iter()
        .fold(u32::MAX, |min_val, val| val.number.min(min_val));
    min_num
}

fn part_two() -> u64 {
    let input = include_str!("input.txt");
    let (seed_str, maps) = input.split_once("\n\n").unwrap(); // split once by empty line

    // extract seeds
    let (_, seed_nrs) = seed_str.split_once(": ").unwrap();
    let seed_nr_strs: Vec<&str> = seed_nrs.split_whitespace().collect();
    let mut seed_ranges = vec![];
    let mut start = true;
    let mut current_idx = 0;
    for seed_num in seed_nr_strs {
        if start {
            let seed_range = SeedRange {
                start: seed_num.parse().expect("Not a number"),
                end: 0,
                converted: false,
            };
            seed_ranges.push(seed_range);
            start = false;
        } else {
            seed_ranges[current_idx].end =
                seed_ranges[current_idx].start + seed_num.parse::<u64>().expect("Not a number");
            current_idx += 1;
            start = true;
        }
    }

    // split into conversion blocks by empty lines
    let blocks = maps.split("\n\n");
    for block in blocks {
        let (_, ranges) = block.split_once("\n").unwrap(); // toss header
        for range in ranges.lines() {
            let mut new_seed_ranges = vec![];
            let (dest_str, back_part) = range.split_once(" ").unwrap();
            let (source_str, range_length_str) = back_part.split_once(" ").unwrap();
            let dest_start: u64 = dest_str.parse().expect("Not a number");
            let conversion_range_start: u64 = source_str.parse().expect("Not a number");
            let conversion_range_length: u64 = range_length_str.parse().expect("Not a number");
            let conversion_range_end: u64 = conversion_range_start + conversion_range_length; // exclusive

            for seed_range in &mut seed_ranges {
                if seed_range.converted {
                    let new_range = SeedRange {
                        start: seed_range.start,
                        end: seed_range.end,
                        converted: seed_range.converted,
                    };
                    new_seed_ranges.push(new_range);
                    continue;
                }

                // if seed range is entirely before conversion range
                if seed_range.end < conversion_range_start {
                    let new_range = SeedRange {
                        start: seed_range.start,
                        end: seed_range.end,
                        converted: seed_range.converted,
                    };
                    new_seed_ranges.push(new_range);
                    continue;
                }

                // if seed range is entirely after conversion range
                if seed_range.start > conversion_range_end {
                    let new_range = SeedRange {
                        start: seed_range.start,
                        end: seed_range.end,
                        converted: seed_range.converted,
                    };
                    new_seed_ranges.push(new_range);
                    continue;
                }

                // now they must overlap

                // possible new range: source before conversion range (not converted)
                if seed_range.start < conversion_range_start {
                    let new_before_range = SeedRange {
                        start: seed_range.start,
                        end: conversion_range_start.min(seed_range.end),
                        converted: false,
                    };
                    new_seed_ranges.push(new_before_range);
                }

                // possible conversion: source within conversion range (adjust start & end accordingly)
                if seed_range.end > conversion_range_start && seed_range.start < conversion_range_end {
                    let start_converting = seed_range.start.max(conversion_range_start);
                    let stop_converting = seed_range.end.min(conversion_range_end);
                    let new_start = dest_start + (start_converting - conversion_range_start);
                    let new_end = dest_start + (stop_converting - conversion_range_start);
                    let new_converted_range = SeedRange {
                        start: new_start,
                        end: new_end,
                        converted: true,
                    };
                    new_seed_ranges.push(new_converted_range);
                }

                // possible new range: source after conversion range
                if seed_range.end > conversion_range_end {
                    let new_after_range = SeedRange {
                        start: conversion_range_end.max(seed_range.start),
                        end: seed_range.end,
                        converted: false,
                    };
                    new_seed_ranges.push(new_after_range);
                }
            }
            seed_ranges = new_seed_ranges;
        }

        // reset conversion markers
        for seed_range in &mut seed_ranges {
            seed_range.converted = false;
        }
    }

    let min_num: u64 = seed_ranges
        .iter()
        .fold(u64::MAX, |min_val, val| val.start.min(min_val));
    min_num
}
