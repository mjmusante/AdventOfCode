// use std::process::exit;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use intcode::Computer;

fn main() {
    let test1 = [
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let mut c1 = Computer::new(&test1.to_vec(), [].to_vec());

    let mut output = vec![];
    loop {
        let next = c1.intcode();
        if !c1.halted() {
            output.push(next.to_string());
        } else {
            break;
        }
    }
    println!("test 1 = {}", output.join(","));

    let test2 = [1102, 34915192, 34915192, 7, 4, 7, 99, 0];
    let mut c2 = Computer::new(&test2.to_vec(), [].to_vec());
    println!("test 2 = {}", c2.intcode());

    let test3 = [104, 1125899906842624, 99];
    let mut c3 = Computer::new(&test3.to_vec(), [].to_vec());
    println!("test 3 = {}", c3.intcode());

    let f = File::open("inputs/day09.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let ary = vlist[0]
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut c = Computer::new(&ary, [1].to_vec());
    println!("part 1 = {}", c.intcode());
}
