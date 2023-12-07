use std::fs::read_to_string;

pub fn run_day() {
    let input =
        to_char_vec(read_to_string("src/day03/input.txt").expect("File could not be opened"));
    let result = part_one_two(&input);
    println!(
        "Day 3 Part 1: The sum of the calibration numbers is {}",
        result.0
    );
    println!("Day 3 Part 1: The sum of the gear ratios is {}", result.1);
    println!()
}

#[derive(Debug)]
struct Star {
    x: usize,
    y: usize,
    adjacent_nrs: usize, // adjacent calibration numbers
    cal_nr_1_idx: usize, // index of first adjacent cal nr in cal nr list
    cal_nr_2_idx: usize, // index of second adjacent cal nr in cal nr list
}

/// Turns passed String into 2D char vec (1st dimension: lines, 2nd dimension: chars in line)
fn to_char_vec(input: String) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line.chars().collect());
    }
    result
}

fn handle_star(stars: &mut Vec<Star>, x: usize, y: usize, cal_nr_idx: usize) {
    let mut found_star = false;
    // check if we've already found something adjacent to this star before
    for star in stars.iter_mut() {
        if star.x == x && star.y == y {
            // if this nr has already been counted for this star, don't do it again
            if cal_nr_idx == star.cal_nr_1_idx || cal_nr_idx == star.cal_nr_2_idx {
                found_star = true;
                continue;
            }
            match star.adjacent_nrs {
                0 => star.cal_nr_1_idx = cal_nr_idx,
                1 => star.cal_nr_2_idx = cal_nr_idx,
                _ => {}
            }
            // count adjacent calibration numbers
            star.adjacent_nrs += 1;
            found_star = true;
        }
    }

    // if this is the first time encountering this star, make a new one
    if !found_star {
        let new_star = Star {
            x: x,
            y: y,
            adjacent_nrs: 1,
            cal_nr_1_idx: cal_nr_idx,
            cal_nr_2_idx: 0,
        };
        stars.push(new_star);
    }
}

fn check_adjacency(
    input: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    cal_nr_idx: usize,
    stars: &mut Vec<Star>,
) -> bool {
    // check coordinates from x-1 to x+1 and y-1 to y+1
    for i in -1..2 {
        let y_idx_signed = y as i32 + i;
        let mut y_idx = 0;
        if y_idx_signed > 0 {
            y_idx = y_idx_signed as usize; // usize can't be negative, so y=0 minus one must be handled
        }
        for j in -1..2 {
            let x_idx_signed = x as i32 + j;
            let mut x_idx = 0;
            if x_idx_signed > 0 {
                x_idx = x_idx_signed as usize; // usize can't be negative, so x=0 minus one must be handled
            }
            match input.get(y_idx) {
                // try to get input at coordinates, if index out of bounds do nothing
                Some(vec) => match vec.get(x_idx) {
                    Some(c) => {
                        // for part two: check if star is gear and note calibration numbers
                        // cal nrs are noted as indexes in list of cal nrs since the full nr isn't necessarily known at this point
                        if *c == '*' {
                            handle_star(stars, x_idx, y_idx, cal_nr_idx);
                        }

                        // for part one: check if adjacent to a non-digit symbol that is not '.'
                        // in this case, it is a calibration nr, can return true
                        if *c != '.' && !c.is_digit(10) {
                            return true;
                        }
                    }
                    None => {}
                },
                None => {}
            }
        }
    }
    // if all adjacencies are checked, this particular digit does not make the nr a calibration nr, return false
    false
}

fn part_one_two(input: &Vec<Vec<char>>) -> (u32, u32) {
    let mut calibration_sum = 0;

    let mut stars: Vec<Star> = Vec::new(); // list of found stars, used to find gears for part 2
    let mut cal_nrs: Vec<u32> = Vec::new(); // list of found calibration nrs, used for gear ratios later

    let mut current_number = 0; // for putting together the current number
    let mut is_cal_nr = false; // whether current nr is a calibration nr
    let mut cal_nr_idx = 0; // index of the current cal nr in the cal nr list

    // check input char by char
    for (y, line) in input.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char.is_digit(10) {
                // put together current number: move one order of magnitude (*10) and add new digit
                current_number = current_number * 10 + char.to_digit(10).expect("Not a digit");
                if check_adjacency(input, x, y, cal_nr_idx, &mut stars) {
                    // adjacent symbol found, this is a calibration nr
                    is_cal_nr = true;
                }
            } else {
                // done with the number (no more digits, got period or other symbol)
                // -> if it's a calibration number, add it to sum and list, increment index
                if is_cal_nr {
                    calibration_sum += current_number;
                    is_cal_nr = false; // reset
                    cal_nr_idx += 1;
                    cal_nrs.push(current_number);
                }
                current_number = 0; // reset number
            }
        }
        // done with the number (line ended)
        // -> if it's a calibration number, add it to sum and list, increment index
        if is_cal_nr {
            calibration_sum += current_number;
            is_cal_nr = false; // reset
            cal_nr_idx += 1;
            cal_nrs.push(current_number);
        }
        current_number = 0; // reset number
    }

    // part two:
    let mut ratio_sum = 0; // sum of gear ratios

    // go through all stars we found
    for star in stars.iter_mut() {
        // gear must have exactly two adjacent cal nrs
        if star.adjacent_nrs == 2 {
            // get cal nrs from the list by index
            let first_cal = cal_nrs
                .get(star.cal_nr_1_idx)
                .expect("Cal nr index not found");
            let second_cal = cal_nrs
                .get(star.cal_nr_2_idx)
                .expect("Cal nr index not found");
            ratio_sum += first_cal * second_cal; // add gear ratio to sum
        }
    }

    (calibration_sum, ratio_sum)
}
