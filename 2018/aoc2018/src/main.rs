extern crate itertools;
extern crate regex;

use std::env;
use std::time::{Duration, Instant};

mod lines;

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

fn main() {
    let mut which = 0;
    if env::args().len() > 1 {
        which = env::args().collect::<Vec<_>>()[1].parse::<usize>().unwrap();
    }

    let lst = [
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
    ];
    let mut n = 1;
    let mut total_time = Duration::new(0, 0);
    for l in lst.iter() {
        if which == 0 || which == n {
            let start = Instant::now();
            let (p1, p2) = l();
            let elapsed = start.elapsed();

            total_time += elapsed;

            println!("----- day {} -----", n);
            println!("Part 1: {}", p1);
            println!("Part 2: {}", p2);
            println!(
                "Execution time: {}ms",
                (elapsed.as_secs() * 1_000) + elapsed.subsec_millis() as u64
            );
        }
        n += 1;
    }
    if which == 0 {
        println!(
            "Total exeuction time: {}ms",
            (total_time.as_secs() * 1_000 + total_time.subsec_millis() as u64)
        );
    }
}
