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

    let mut c1 = Computer::new(&ary, [1].to_vec());

    let mut loop_count = 0;
    let part1;

    loop {
        loop_count += 1;
        let rslt = c1.intcode();
        if c1.halted() {
            part1 = rslt;
            break;
        }
        if rslt != 0 {
            println!("Got error on loop count {} at ip {}", loop_count, c1.current_ip());
            exit(1);
        }
    }

    let mut c5 = Computer::new(&ary, [5].to_vec());
    let part2 = c5.intcode();

    println!("part 1 = {}", part1);
    println!("part 2 = {}", part2);
}
