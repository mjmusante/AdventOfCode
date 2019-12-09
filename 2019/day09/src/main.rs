// use std::process::exit;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use intcode::Computer;

fn main() {
    let f = File::open("inputs/day09.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let ary = vlist[0]
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut part1 = Computer::new(&ary).with_input([1].to_vec());
    println!("part 1 = {}", part1.intcode());

    let mut part2 = Computer::new(&ary).with_input([2].to_vec());
    println!("part 2 = {}", part2.intcode());
}
