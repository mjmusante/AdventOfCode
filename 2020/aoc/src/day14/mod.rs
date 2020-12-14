use regex::Regex;
use std::collections::HashMap;

use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/14.txt");

    println!("Part 1 = {}", execute(&lines));
}

fn execute(program: &Vec<String>) -> i64 {
    let mut and = 0;
    let mut or = 0;
    let mut mem = HashMap::<usize, i64>::new();
    let reg_mem = Regex::new(r"^mem\[(\d+)\] = (\d+)").unwrap();

    for instr in program {
        if instr.starts_with("mask") {
            and = (1 << 36) - 1;
            or = 0;
            let mut pos = 1 << 35;
            for ch in instr[7..].chars() {
                match ch {
                    'X' => (),
                    '1' => or |= pos,
                    '0' => and &= !pos,
                    _ => {
                        panic!("Invalid char in mask");
                    }
                }
                pos >>= 1;
            }
        } else {
            let cap = reg_mem
                .captures_iter(instr)
                .next()
                .expect("invalid instruction");
            let addr: usize = cap[1].parse().expect("bad memory address");
            let data: i64 = cap[2].parse().expect("bad data value");
            mem.insert(addr, data & and | or);
        }
    }

    mem.values().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_data() -> Vec<String> {
        vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string(),
        ]
    }

    #[test]
    fn test1() {
        let v = test_data();
        assert_eq!(execute(&v), 165);
    }
}
