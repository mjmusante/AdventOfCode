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

    for i in 1..=2 {
        let mut c = Computer::new(&ary).with_input([i].to_vec());
        println!("part {} = {}", i, c.run());
    }
}
