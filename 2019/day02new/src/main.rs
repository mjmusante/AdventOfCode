use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use intcode::Computer;

fn main() {
    let f = File::open("inputs/day02.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let ary = vlist[0]
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut c = Computer::new(&ary);

    c.set_noun_verb(12, 2);
    let part1 = c.run();

    let mut part2 = -1;
    'outer: for i in 0..=99 {
        for j in 0..=99 {
            let mut c = Computer::new(&ary);
            c.set_noun_verb(i, j);
            if c.run() == 19690720 {
                part2 = 100 * i + j;
                break 'outer;
            }
        }
    }

    println!("part 1 = {}", part1);
    println!("part 2 = {}", part2);
}
