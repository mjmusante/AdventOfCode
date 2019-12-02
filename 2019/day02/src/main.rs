use std::process::exit;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("data/input.txt").unwrap();
    let vlist =  BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let ary = vlist[0]
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    
    let mut part1 = ary.clone();
    let mut pos = 0;

    part1[1] = 12;
    part1[2] = 2;

    loop {
        let (val, loc) = match part1[pos] {
            1 => { pos += 4; (part1[part1[pos - 3] as usize] + part1[part1[pos - 2] as usize], part1[pos - 1]) }
            2 => { pos += 4; (part1[part1[pos - 3] as usize] * part1[part1[pos - 2] as usize], part1[pos - 1]) }
            99 => { (-1, -1) }
            _ => { println!("failure; got {} as a value", part1[pos]); exit(1); }
        };

        if loc < 0 {
            break;
        }
        part1[loc as usize] = val
    }
    println!("val at 0 is {}", part1[0]);
}
