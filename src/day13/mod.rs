pub fn run_day() {
    let input_str = include_str!("input.txt");
    let input = parse_input(input_str);

    println!(
        "Day 13 Part 1: Sum of adjusted reflection lines is {}",
        part_one(&input)
    );
    println!(
        "Day 13 Part 2: With smudges fixed, sum is {}",
        part_two(&input)
    );

    println!();
}

fn parse_input(input_str: &str) -> Vec<Vec<Vec<char>>> {
    input_str
        .split("\n\n")
        .map(|block| block.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

fn part_one(input: &Vec<Vec<Vec<char>>>) -> usize {
    input.iter().map(|map| find_reflection(map, false, 0).expect("No reflection found in part 1!")).sum()
}

fn part_two(input: &Vec<Vec<Vec<char>>>) -> usize {
    // save maps with original reflections to check against later
    let input_with_reflections: Vec<(Vec<Vec<char>>, usize)> = input.iter().map(|map| (map.to_owned(), find_reflection(map, false, 0).expect("No reflection found in part 1!"))).collect();

    let mut reflection_sum = 0;
    for (map, reflection_line) in input_with_reflections {
        let mut success = false;
        // try out each coordinate for fixing smudge
        for row in 0..map.len() {
            for col in 0..map[row].len() {
                let mut new_map = map.clone();
                new_map[row][col] = match new_map[row][col] {
                    '.' => '#',
                    '#' => '.',
                    _ => panic!("Unknown symbol!")
                };
                // check if there is now a new reflection line
                match find_reflection(&new_map, true, reflection_line) {
                    Some(result) => {
                        success = true;
                        reflection_sum += result;
                        break;
                    },
                    None => {}
                }
            }
            if success {
                break;
            }
        }
        if !success {
            let map_str = map.iter().map(|row| row.iter().collect::<String>() + "\n").collect::<String>();
            panic!("Couldn't fix smudge for map:\n{map_str}");
        }
    }

    reflection_sum
}

/// finds reflection in map, either horizontally or vertically
/// returns line nr after reflection for vertical, line nr after reflection * 100 for horizontal
fn find_reflection(map: &Vec<Vec<char>>, part2: bool, cannot_be: usize) -> Option<usize> {
    // check horizontally
    // check for duplicated lines, then verify whether it's a complete reflection for found pairs
    let horizontal_matches: Vec<usize> = map
        .windows(2)
        .enumerate()
        .filter(|(idx, pair)| pair[0] == pair[1] && verify_reflection(*idx+1, &map))
        .map(|(idx, _)| idx+1)
        .collect();

    if !horizontal_matches.is_empty() {
        // check each found reflection line against the line it's not allowed to be for part 2
        // if part2 is false this will simply return the first (and presumably only) reflection line found
        for h_match in horizontal_matches {
            if !part2 || (h_match * 100) != cannot_be {
                return Some(h_match * 100);
            }
        }
    }

    // check vertically
    // transpose map and repeat horizontal process
    let map = transpose(map.to_vec());

    let vertical_matches: Vec<usize> = map
        .windows(2)
        .enumerate()
        .filter(|(idx, pair)| pair[0] == pair[1] && verify_reflection(*idx+1, &map))
        .map(|(idx, _)| idx+1)
        .collect();

    if !vertical_matches.is_empty() {
        // check each found reflection line against the line it's not allowed to be for part 2
        // if part2 is false this will simply return the first (and presumably only) reflection line found
        for v_match in vertical_matches {
            if !part2 || v_match != cannot_be {
                return Some(v_match);
            }
        }
    }

    None
}

fn verify_reflection(index: usize, map: &Vec<Vec<char>>) -> bool {
    for i in 0..map.len() {
        if (index as i32 - 1 - i as i32) < 0 {
            // reached top end of map
            return true;
        }

        // compare two lines, moving upwards and downwards from the initial pair
        match (map.get(index + i), map.get(index - 1 - i)) {
            (_, None) => return true, // reached end of map without finding line that isn't reflected
            (None, _) => return true, // reached end of map without finding line that isn't reflected
            (Some(row1), Some(row2)) => {
                if row1 != row2 {
                    // rows don't match, not a true reflection
                    return false;
                }
            }
        }
    }
    
    // made it all the way through the map without finding a line that isn't reflected
    true
}

/// transposes 2D char vec
fn transpose(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut transposed_map: Vec<Vec<char>> = (0..map[0].len()).map(|_| vec![]).collect();

    for line in map {
        for (item, transposed_row) in line.into_iter().zip(&mut transposed_map) {
            transposed_row.push(item);
        }
    }

    transposed_map
}