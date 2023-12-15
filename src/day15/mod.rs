pub fn run_day() {
    let input_str = include_str!("input.txt");
    let input = parse_input(input_str);
    println!("Day 15 Part 1: Sum of hashes is {}", part_one(&input));
    println!("Day 15 Part 2: Sum of focusing power is {}", part_two(&input));
    println!()
}

fn parse_input(input_str: &str) -> Vec<String> {
    input_str
        .split(",")
        .map(|step| step.strip_suffix("\n").unwrap_or(step).to_string())
        .collect()
}

fn part_one(input: &Vec<String>) -> u32 {
    input.into_iter().map(|step| hash(&step)).sum()
}

fn part_two(input: &Vec<String>) -> usize {
    let mut hash_table: Vec<Vec<(String, u32)>> = (0..256).map(|_| vec![]).collect();

    for lens in input {
        let lens_bytes = lens.as_bytes();
        let op_idx = lens.find(|c| c == '-' || c == '=').unwrap();
        let hash = hash(&lens[..op_idx]) as usize;

        match lens_bytes[op_idx] as char {
            '-' => {
                hash_table[hash] = hash_table[hash]
                    .iter()
                    .filter(|entry| entry.0 != &lens[..op_idx])
                    .map(|entry| (entry.0.to_string(), entry.1))
                    .collect()
            }
            '=' => {
                let focal_length: u32 = (lens_bytes[op_idx + 1] as char).to_string().parse().unwrap();
                match hash_table[hash].iter().any(|entry| entry.0 == &lens[..op_idx]) {
                    true => {
                        hash_table[hash] = hash_table[hash]
                            .iter()
                            .map(|entry| {
                                if entry.0 == &lens[..op_idx] {
                                    (entry.0.to_string(), focal_length)
                                } else {
                                    (entry.0.to_string(), entry.1)
                                }
                            })
                            .collect()
                    }
                    false => hash_table[hash].push((lens[..op_idx].to_string(), focal_length)),
                }
            }
            _ => panic!("Unknown operation {} in {lens}!", (lens_bytes[op_idx] as char)),
        }
    }

    let mut focusing_sum = 0;

    for (hash, lens_box) in hash_table.iter().enumerate() {
        for (pos, lens) in lens_box.iter().enumerate() {
            focusing_sum += (hash + 1) * (pos + 1) * (lens.1 as usize);
        }
    }

    focusing_sum
}

fn hash(step: &str) -> u32 {
    let mut hash = 0;

    for c in step.chars() {
        hash = ((hash + c as u32) * 17) % 256;
    }

    hash
}
