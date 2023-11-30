use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if 2 == args.len() {
        aoc_2023::get_days()[args[1].parse::<usize>().unwrap() - 1]();
    } else {
        for call in aoc_2023::get_days() {
            call();
        }
    }
}