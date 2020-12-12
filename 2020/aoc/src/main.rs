use core::time::Duration;
use std::time::Instant;

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

type Func = fn();

fn main() {
    let runners: Vec<Func> = vec![
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
        day07::run,
        day08::run,
        day09::run,
        day10::run,
        day11::run,
        day12::run,
    ];
    let mut total = Duration::new(0, 0);

    for run in runners {
        let now = Instant::now();
        run();
        let elapsed = now.elapsed();

        total += elapsed;
        println!(" -> Time: {}ms", elapsed.as_millis());
    }
    println!("\nTotal time: {}ms", total.as_millis());
}
