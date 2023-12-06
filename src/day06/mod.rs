struct Race {
    time: u64,
    distance: u64
}

const INPUT:[Race; 4] = [Race{time: 53, distance: 313}, Race{time: 89, distance: 1090}, Race{time: 76, distance: 1214}, Race{time: 98, distance: 1201}];

const LONG_RACE: Race = Race{time: 53897698, distance: 313109012141201};

pub fn run_day() {
    println!("Day 6 Part 1: Product of options to win each race is {}", part_one());
    println!("Day 6 Part 2: There are {} ways to win the long race", part_two());
}

fn part_one() -> u64 {
    let mut winning_option_product = 1;

    for race in INPUT {
        let mut winning_options = 0;
        for speed in 1..race.time {
            let running_time= race.time - speed;
            let distance_traveled = speed * running_time;
            if distance_traveled > race.distance {
                winning_options += 1;
            }
        }
        winning_option_product *= winning_options;
    }

    winning_option_product
}

fn part_two() -> u64 {
    let mut winning_options = 0;

    for speed in 1..LONG_RACE.time {
        let running_time= LONG_RACE.time - speed;
        let distance_traveled = speed * running_time;
        if distance_traveled > LONG_RACE.distance {
            winning_options += 1;
        }
    }
    winning_options
}
