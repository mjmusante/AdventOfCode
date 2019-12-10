use std::process::exit;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use intcode::Computer;

fn main() {
    let f = File::open("inputs/day05.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let ary = vlist[0]
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let part1;
    let mut c1 = Computer::new(&ary).with_input(vec![1]);
    c1.run();
    assert!(c1.halted());
    loop {
        if !c1.has_output() {
            println!("Computer[1] failed to generate output");
            exit(1);
        }
        let val = c1.next_output();
        if val != 0 {
            part1 = val;
            break;
        }
    }

    let mut c5 = Computer::new(&ary).with_input(vec![5]);
    c5.run();
    if !c5.has_output() {
        println!("Computer[5] failed to generate output");
        exit(1);
    }
    assert!(c5.halted());
    let part2 = c5.next_output();

    println!("part 1 = {}", part1);
    println!("part 2 = {}", part2);
}
