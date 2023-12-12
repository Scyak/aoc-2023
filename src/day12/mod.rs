use cached::proc_macro::cached;

#[derive(Debug)]
struct SpringData {
    data_str: String,
    groups: Vec<u32>,
}

pub fn run_day() {
    let input_str = include_str!("input.txt");
    let input = parse_input(input_str);

    let unfolded = unfold_data(&input);

    println!(
        "Day 12 Part 1: {} options for broken spring configurations",
        part_one_two(input)
    );
    println!(
        "Day 12 Part 2: {} options for unfolded data",
        part_one_two(unfolded)
    );

    println!();
}

fn parse_input(input_str: &str) -> Vec<SpringData> {
    input_str
        .lines()
        .map(|line| {
            let (data, group_str) = line.split_once(" ").unwrap();
            let groups = group_str.split(",").map(|c| c.parse().unwrap()).collect();
            SpringData {
                data_str: data.to_string(),
                groups: groups,
            }
        })
        .collect()
}

fn unfold_data(input: &Vec<SpringData>) -> Vec<SpringData> {
    input
        .iter()
        .map(|data| SpringData {
            data_str: (data.data_str.to_string() + "?").repeat(4) + &data.data_str,
            groups: data.groups[..].repeat(5)
        })
        .collect()
}

fn part_one_two(input: Vec<SpringData>) -> u64 {
    let mut option_sum = 0;
    for data in input.into_iter() {
        let result = valid_option_count(data.data_str.to_string(), data.groups);
        option_sum += result as u64;
    }
    option_sum
}

// turns out Rust can do the caching for you!
#[cached]
fn valid_option_count(mut data_str: String, groups: Vec<u32>) -> u64 {
    // remove leading or trailing periods    
    data_str = data_str.trim_matches('.').to_string();

    if groups.is_empty() {
        if !data_str.contains("#") {
            // groups empty, string done, great
            return 1;
        } else {
            // still broken springs, but no more groups!
            return 0;
        }
    }
    
    // string is done and we're looking for more groups, can't succeed
    if data_str.is_empty() {
        return 0;
    }

    // if group started (# at beginning of string): try to complete group
    // completing group is impossible if:
    // - string too short
    // - periods in group length
    // - # where group should be over
    if data_str.starts_with("#") {
        if data_str.len() < groups[0] as usize || data_str[..groups[0] as usize].chars().any(|c| c == '.') || data_str.chars().nth(groups[0] as usize) == Some('#') {
            return 0;
        } else {
            // completing group is possible, cut out this group and check the rest
            if groups.len() > 1 {
                if data_str.len() > groups[0] as usize {
                    // string from start to group[0]+1 is forced ({#}.)
                    data_str = data_str[(groups[0] + 1) as usize..].to_string();
                    return valid_option_count(data_str, groups[1..].to_vec());
                } else {
                    // need to keep searching, but string is too short
                    return 0;
                }
            } else {
                // no more groups, but remaining string still needs to be checked (may have extra #s)
                // don't continue from groups[0]+1 as that index may not exist (could be ., could be end of string)
                data_str = data_str[groups[0] as usize..].to_string();
                return valid_option_count(data_str, groups[1..].to_vec());
            }
        }
    }

    // no question marks means nothing to replace, just need to finish going through
    if !data_str.contains("?") {
        return valid_option_count(data_str[1..].to_string(), groups);
    }

    // try both options for ?
    let working_str = data_str.replacen("?", ".", 1);
    let broken_str = data_str.replacen("?", "#", 1);
    let valid_count = valid_option_count(working_str, groups.clone())
        + valid_option_count(broken_str, groups.clone());

    return valid_count;
}
