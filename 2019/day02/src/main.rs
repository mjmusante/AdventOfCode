use std::process::exit;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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
    let f = File::open("data/input.txt").unwrap();
    let vlist =  BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let ary = vlist[0]
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    println!("part 1 = {}", try_values(&ary, 12, 2));
    for i in 0..99 {
        for j in 0..99 {
            let v = try_values(&ary, i, j);
            if v == 19690720 {
                println!("part 2 = noun: {}, verb: {}, code = {}", i, j, 100 * i + j);
                exit(0);
            }
        }
    }
    println!("part 2 = FAILED");
}
