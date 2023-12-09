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
    let mut point_sum = 0;
    let input = read_to_string("src/day04/input.txt").unwrap();

    for line in input.lines() {
        let card_nrs = line.split(":").collect::<Vec<&str>>();
        let card_nr_str = card_nrs.get(1).expect("Nothing on card");
        let split_numbers = card_nr_str.split("|").collect::<Vec<&str>>();
        let winning_nrs = split_numbers.get(0).expect("No winning nrs found");
        let mut ticket_points = 0;
        for scratched_nr in split_numbers
            .get(1)
            .expect("No numbers on ticket")
            .split_whitespace()
        {
            let nr_with_space = " ".to_owned() + scratched_nr + " ";
            if winning_nrs.contains(&nr_with_space) {
                if ticket_points == 0 {
                    ticket_points = 1;
                } else {
                    ticket_points *= 2;
                }
            }
        }
        point_sum += ticket_points;
    }

    point_sum
}

fn part_two() -> usize {
    let mut tickets: [usize; 207] = [1; 207];
    let mut ticket_sum = 0;
    let input = read_to_string("src/day04/input.txt").unwrap();

    let mut current_ticket_nr = 0;

    for line in input.lines() {
        let card_nrs = line.split(":").collect::<Vec<&str>>();
        let card_nr_str = card_nrs.get(1).expect("Nothing on card");
        let split_numbers = card_nr_str.split("|").collect::<Vec<&str>>();
        let winning_nrs = split_numbers.get(0).expect("No winning nrs found");

        let mut ticket_matches = 0;
        for scratched_nr in split_numbers
            .get(1)
            .expect("No numbers on ticket")
            .split_whitespace()
        {
            let nr_with_space = " ".to_owned() + scratched_nr + " ";
            if winning_nrs.contains(&nr_with_space) {
                ticket_matches += 1;
            }
        }
        for i in 1..(ticket_matches+1) {
            tickets[current_ticket_nr+i] += tickets[current_ticket_nr];
        }
        println!("Adding {} tickets for ticket {current_ticket_nr}", tickets[current_ticket_nr]);
        ticket_sum += tickets[current_ticket_nr];
        current_ticket_nr += 1;
    }

    ticket_sum
}
