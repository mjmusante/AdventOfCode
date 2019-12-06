use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::time::Instant;

fn try_values(program: &Vec<i64>, noun: i64, verb: i64) -> i64 {
    let mut mem = program.clone();
    let mut pos = 0;

    mem[1] = noun;
    mem[2] = verb;

    loop {
        let (val, loc) = match mem[pos] {
            1 => { pos += 4; (mem[mem[pos - 3] as usize] + mem[mem[pos - 2] as usize], mem[pos - 1]) }
            2 => { pos += 4; (mem[mem[pos - 3] as usize] * mem[mem[pos - 2] as usize], mem[pos - 1]) }
            99 => { (-1, -1) }
            _ => { return -1; }
        };

        if loc < 0 {
            break;
        }
        mem[loc as usize] = val
    }

    mem[0]
}


fn main() {
    let f = File::open("inputs/day02.txt").unwrap();
    let vlist =  BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let ary = vlist[0]
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let now = Instant::now();

    let part1 = try_values(&ary, 12, 2);
    let mut part2 = 0;

    'outer: for i in 0..99 {
        for j in 0..99 {
            let v = try_values(&ary, i, j);
            if v == 19690720 {
                part2 = 100 * i + j;
                break 'outer;
            }
        }
    }
    let exec_ms = now.elapsed().as_millis();

    println!("part 1 = {}", part1);
    println!("part 2 = {}", if part2 == 0 { "FAILED".to_string() } else {part2.to_string()} );
    println!("elapsed time = {}ms", exec_ms);
}
