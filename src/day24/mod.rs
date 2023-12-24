use regex::Regex;

#[derive(Debug, Clone)]
struct Hailstone {
    position: (i128, i128, i128),
    speed: (i128, i128, i128),
}

pub fn run_day() {
    let input_str = include_str!("input.txt");
    let hailstones = parse_input(input_str);
    println!(
        "Day 24 Part 1: {} intersections in test area",
        part_one(hailstones.clone(), 200000000000000.0, 400000000000000.0)
    );
    println!(
        "Day 24 Part 2: Sum of stone coordinates is {}",
        part_two(&hailstones)
    );
}

fn parse_input(input_str: &str) -> Vec<Hailstone> {
    let re = Regex::new(r"(\d+), *(\d+), *(\d+) *@ *(-?\d+), *(-?\d+), *(-?\d+)").unwrap();
    re.captures_iter(input_str)
        .map(|caps| {
            let (_, [x, y, z, x_speed, y_speed, z_speed]) = caps.extract();
            Hailstone {
                position: (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap()),
                speed: (
                    x_speed.parse().unwrap(),
                    y_speed.parse().unwrap(),
                    z_speed.parse().unwrap(),
                ),
            }
        })
        .collect()
}

fn part_one(mut hailstones: Vec<Hailstone>, test_area_min: f64, test_area_max: f64) -> u32 {
    let mut intersections = 0;

    while let Some(hailstone) = hailstones.pop() {
        for other_hail in hailstones.iter() {
            if let Some(intersection) = xy_intersection(&hailstone, other_hail) {
                if intersection.0 >= test_area_min
                    && intersection.0 <= test_area_max
                    && intersection.1 >= test_area_min
                    && intersection.1 <= test_area_max
                {
                    intersections += 1;
                }
            }
        }
    }

    intersections
}

fn part_two(hailstones: &Vec<Hailstone>) -> i128 {
    let mut stone_start_xy = (0, 0);
    let mut stone_z = 0;

    let mut last_search_to = 0;
    let mut search_to = 10;
    let mut success = false;

    // this works as follows:
    // - try options for x and y velocity of the stone roughly in order of ascending magnitude
    //      (start with -10 to 10 for x and y each, then do -20 to 20 - skipping -10 to 10 - and so on)
    // - for each velocity option:
    //      - adjust speeds of the hailstones so stone is stationary and hail is moving relative to stone
    //      - try to find xy point where all hailstones xy-intersect (this must be where the stone starts, as it does not move in this relative version)
    //      - if even one hailstone does not intersect, this velocity cannot work, try the next one
    //      - once you've found an intersection point, check if working z coordinate exists (it almost certainly does, but you gotta check)
    //      - add up the coordinates you've found and you're done!
    while !success {
        for delta_x in (search_to * -1)..search_to {
            for delta_y in (search_to * -1)..search_to {
                // skip what you've already done
                if i128::abs(delta_x) < last_search_to && i128::abs(delta_y) < last_search_to {
                    // already checked in a previous loop, don't do it again
                    continue;
                }

                // adjust hailstone velocity vectors by the velocity we're trying for the stone
                let adjusted_hailstones: Vec<Hailstone> = hailstones
                    .iter()
                    .map(|hailstone| Hailstone {
                        position: hailstone.position,
                        speed: (hailstone.speed.0 - delta_x, hailstone.speed.1 - delta_y, 0),
                    })
                    .collect();

                // search for xy collisions first, as it's quicker and simpler than checking x, y and z (and already implemented)
                success = true;
                stone_start_xy = (0, 0);

                let hailstone = &adjusted_hailstones[0];

                // see if intersection of the stone we picked out with every other path is at the same point
                // parallel lines count as valid intersections here (as they will definitely also be the same at the point where all others intersect)
                for other_hail in adjusted_hailstones.iter().skip(1) {
                    if let Some(intersection) = xy_intersection(&hailstone, other_hail) {
                        if stone_start_xy != (intersection.0 as i128, intersection.1 as i128)
                            && stone_start_xy != (0, 0)
                            && intersection != (f64::INFINITY, f64::INFINITY)
                        {
                            success = false;
                            break;
                        } else if stone_start_xy == (0, 0)
                            && intersection != (f64::INFINITY, f64::INFINITY)
                        {
                            stone_start_xy = (intersection.0 as i128, intersection.1 as i128);
                        }
                    } else {
                        success = false;
                        break;
                    }
                }

                // if we've made it here without failing, there's a point all hailstones pass in the relative version of events
                if success {
                    // check that a working z coordinate exists

                    // get intercept time and z coordinate of first hailstone
                    let time_to_intercept_0 = (stone_start_xy.0
                        - adjusted_hailstones[0].position.0)
                        / adjusted_hailstones[0].speed.0;
                    let intercept_z_0 =
                        hailstones[0].position.2 + (time_to_intercept_0 * hailstones[0].speed.2);

                    let mut previous_slope = 0;

                    // for each other hailstone: calculate slope of z curve and check it's constant
                    for i in 1..adjusted_hailstones.len() {
                        let time_to_intercept = (stone_start_xy.0
                            - adjusted_hailstones[i].position.0)
                            / adjusted_hailstones[i].speed.0;
                        let intercept_z =
                            hailstones[i].position.2 + (time_to_intercept * hailstones[i].speed.2);

                        let slope = (intercept_z - intercept_z_0)
                            / (time_to_intercept - time_to_intercept_0);

                        if slope != previous_slope && previous_slope != 0 {
                            // guess finding a z didn't work out after all... keep trying other velocities
                            success = false;
                        } else {
                            previous_slope = slope;
                        }
                    }

                    // calculating the stone starting point is now easy, since we have a point with corresponding time and the slope of the z position curve
                    stone_z = intercept_z_0 - (previous_slope * time_to_intercept_0);

                    if success {
                        break;
                    }
                }
            }
            if success {
                break;
            }
        }
        // search larger magnitudes in the next iteration
        last_search_to = search_to;
        search_to += 10;
    }

    stone_start_xy.0 + stone_start_xy.1 + stone_z
}

/// entirely ignores z component of coordinates and speed
/// returns None if intersection is in the past
/// returns (inf, inf) if lines are parallel
/// returns coordinates of xy intersection otherwise
fn xy_intersection(a: &Hailstone, b: &Hailstone) -> Option<(f64, f64)> {
    // parallel lines or the same line
    if vec_multiple((a.speed.0, a.speed.1), (b.speed.0, b.speed.1)) {
        return Some((f64::INFINITY, f64::INFINITY));
    }

    // formula for intersection of two lines given as vector equations
    // lambda refers to this equation for hailstone a: (x, y) = (x0, y0) + lambda * (xspeed, yspeed)
    let lambda = (((a.position.1 - b.position.1) * b.speed.0)
        - ((a.position.0 - b.position.0) * b.speed.1)) as f64
        / ((a.speed.0 * b.speed.1) - (a.speed.1 * b.speed.0)) as f64;

    // now put lambda into hailstone a equation to get coordinates
    let intersection_x = a.position.0 as f64 + (lambda * a.speed.0 as f64);
    let intersection_y = a.position.1 as f64 + (lambda * a.speed.1 as f64);

    let future_a = a.speed.0.signum() == (intersection_x - a.position.0 as f64).signum() as i128;
    let future_b = b.speed.0.signum() == (intersection_x - b.position.0 as f64).signum() as i128;

    if !future_a || !future_b {
        return None;
    }

    Some((intersection_x, intersection_y))
}

// this became a bit of a mess with all the checks for 0, whoops
// could probably be prettier but it's Christmas and I'm not spending more time on this function
fn vec_multiple(a: (i128, i128), b: (i128, i128)) -> bool {
    let a = (i128::abs(a.0), i128::abs(a.1));
    let b = (i128::abs(b.0), i128::abs(b.1));

    if a.0 > b.0 {
        if a.0 == 0 && b.0 == 0 {
            return (b.1 == 0 && a.1 == 0) || (!(b.1 == 0 && a.1 != 0) && (a.1 % b.1 == 0));
        }

        if (b.0 == 0 && a.0 != 0) || a.0 % b.0 != 0 {
            return false;
        }
        let factor = a.0 / b.0;
        return (b.1 == 0 && a.1 == 0)
            || (!(b.1 == 0 && a.1 != 0) && (a.1 % b.1 == 0) && (a.1 / factor == b.1));
    } else {
        if a.0 == 0 && b.0 == 0 {
            return (a.1 == 0 && b.1 == 0) || (!(a.1 == 0 && b.1 != 0) && (b.1 % a.1 == 0));
        }

        if (a.0 == 0 && b.0 != 0) || b.0 % a.0 != 0 {
            return false;
        }
        let factor = b.0 / a.0;
        return (b.1 == 0 && a.1 == 0)
            || (!(a.1 == 0 && b.1 != 0) && (b.1 % a.1 == 0) && (b.1 / factor == a.1));
    }
}
