mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub fn get_days() -> Vec<fn()> {
    vec![
        day01::run_day,
        day02::run_day,
        day03::run_day,
        day04::run_day,
        day05::run_day,
        day06::run_day,
        day07::run_day,
        day08::run_day,
        day09::run_day,
        day10::run_day,
        day11::run_day,
        day12::run_day,
        day13::run_day,
        day14::run_day,
        day15::run_day,
        day16::run_day,
        day17::run_day,
        day18::run_day,
        day19::run_day,
        day20::run_day,
        day21::run_day,
        day22::run_day,
        day23::run_day,
        day24::run_day,
        day25::run_day,
    ]
}