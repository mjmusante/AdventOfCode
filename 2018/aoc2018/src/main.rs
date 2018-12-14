extern crate itertools;
extern crate regex;

use std::env;

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
        day10::run
    ];
    let mut n = 1;
    for l in lst.iter() {
        if which == 0 || which == n {
            let (p1, p2) = l();
            println!("----- day {} -----", n);
            println!("Part 1: {}", p1);
            println!("Part 2: {}", p2);
        }
        n += 1;
    }
}
