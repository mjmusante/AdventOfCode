extern crate itertools;
extern crate regex;

mod lines;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    println!("----- day 01 -----");
    day01::run();
    println!("----- day 02 -----");
    day02::run();
    println!("----- day 03 -----");
    day03::run();
    println!("----- day 04 -----");
    day04::run();
}
