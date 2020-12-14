use regex::Regex;
use std::collections::HashMap;
use std::iter;

use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/14.txt");

    println!("Part 1 = {}", execute(&lines, 1));
    println!("Part 2 = {}", execute(&lines, 2));
}

fn execute(program: &Vec<String>, part: i64) -> i64 {
    let mut and = 0;
    let mut or = 0;
    let mut fulland = 0;
    let mut xlist = Vec::<i64>::new();
    let mut orvec = Vec::<i64>::new();
    let mut mem = HashMap::<i64, i64>::new();
    let reg_mem = Regex::new(r"^mem\[(\d+)\] = (\d+)").unwrap();

    for instr in program {
        if instr.starts_with("mask") {
            and = (1 << 36) - 1;
            or = 0;
            xlist = Vec::<i64>::new();
            let mut pos = 1 << 35;
            for ch in instr[7..].chars() {
                match ch {
                    'X' => xlist.push(pos),
                    '1' => or |= pos,
                    '0' => and &= !pos,
                    _ => {
                        panic!("Invalid char in mask");
                    }
                }
                pos >>= 1;
            }

            if xlist.len() > 0 && xlist.len() < 12 {
                fulland = xlist.iter().fold((1 << 36) - 1, |a, v| a & !v);
                orvec = iter::repeat(or)
                    .take(1 << xlist.len())
                    .collect::<Vec<i64>>();

                for i in 0..xlist.len() {
                    for j in 0..(1 << xlist.len()) {
                        if (1 << i) & j != 0 {
                            orvec[j] |= xlist[i];
                        }
                    }
                }
            }
        } else {
            let cap = reg_mem
                .captures_iter(instr)
                .next()
                .expect("invalid instruction");
            let addr: i64 = cap[1].parse().expect("bad memory address");
            let data: i64 = cap[2].parse().expect("bad data value");

            if part == 1 {
                mem.insert(addr, data & and | or);
            } else if part == 2 {
                for i in 0..(1 << xlist.len()) {
                    let newaddr = addr & fulland | orvec[i];
                    mem.insert(newaddr, data);
                }
            }
        }
    }

    mem.values().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_data1() -> Vec<String> {
        vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
            "mem[8] = 11".to_string(),
            "mem[7] = 101".to_string(),
            "mem[8] = 0".to_string(),
        ]
    }

    fn test_data2() -> Vec<String> {
        vec![
            "mask = 000000000000000000000000000000X1001X".to_string(),
            "mem[42] = 100".to_string(),
            "mask = 00000000000000000000000000000000X0XX".to_string(),
            "mem[26] = 1".to_string(),
        ]
    }

    #[test]
    fn test1() {
        let v = test_data1();
        assert_eq!(execute(&v, 1), 165);
    }

    #[test]
    fn test2() {
        let v = test_data2();
        assert_eq!(execute(&v, 2), 208);
    }
}
