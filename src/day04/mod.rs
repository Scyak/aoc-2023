use std::fs::read_to_string;

pub fn run_day() {
    println!(
        "Day 4 Part 1: The sum of the scratchcard points is {}",
        part_one()
    );
    println!(
        "Day 4 Part 2: The total number of scratch cards is {}",
        part_two()
    );
}

fn part_one() -> u32 {
    let mut point_sum = 0; // counts total points
    let input = read_to_string("src/day04/input.txt").unwrap();

    for line in input.lines() {
        let (_, card_nrs) = line.split_once(":").unwrap(); // split off ticket number
        let (winning_nrs, scratched_nrs) = card_nrs.split_once("|").unwrap(); // split winning numbers and numbers we have
        let mut ticket_points = 0; // counts points for this ticket only

        // check each number we have
        for scratched_nr in scratched_nrs.split_whitespace() {
            // space before and after to ensure e.g. 76 is not recognized as 7 etc.
            let nr_with_space = " ".to_owned() + scratched_nr + " ";
            if winning_nrs.contains(&nr_with_space) {
                if ticket_points == 0 {
                    ticket_points = 1; // if we had 0 numbers, we get 1 point now
                } else {
                    ticket_points *= 2; // else double the score
                }
            }
        }
        point_sum += ticket_points; // add ticket sum to total sum
    }

    point_sum
}

fn part_two() -> usize {
    let mut ticket_sum = 0; // total number of tickets overall
    let input = include_str!("input.txt");
    let mut tickets: Vec<usize> = vec![1; input.lines().count()]; // counts amount of each ticket

    let mut current_ticket_nr = 0; // ticket we're currently handling

    for line in input.lines() {
        let (_, card_nrs) = line.split_once(":").unwrap(); // split off ticket number
        let (winning_nrs, scratched_nrs) = card_nrs.split_once("|").unwrap(); // split winning numbers and numbers we have
        let mut ticket_matches = 0; // counts matches for this ticket

        // check each number we have
        for scratched_nr in scratched_nrs.split_whitespace() {
            // space before and after to ensure e.g. 76 is not recognized as 7 etc.
            let nr_with_space = " ".to_owned() + scratched_nr + " ";
            if winning_nrs.contains(&nr_with_space) {
                ticket_matches += 1; // count matches
            }
        }
        // Each match n gets us copies of ticket current+n
        for i in 1..(ticket_matches + 1) {
            // number of copies gained per ticket is equal to number of copies we have of the current ticket
            tickets[current_ticket_nr + i] += tickets[current_ticket_nr];
        }
        ticket_sum += tickets[current_ticket_nr]; // done with this ticket now, can add to sum
        current_ticket_nr += 1; // get ready for next ticket
    }

    ticket_sum
}
