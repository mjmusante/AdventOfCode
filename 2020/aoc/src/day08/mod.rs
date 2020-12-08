use regex::Regex;
use std::collections::HashSet;

use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/08.txt");
    let prog = parse_file(&lines);

    println!("Part 1 = {}", execute(&prog, false));
    println!("Part 2 = {}", execute(&prog, true));
}

#[derive(Debug, Eq, PartialEq)]
enum Opcode {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug)]
struct Instruction {
    op: Opcode,
    arg: i64,
}

fn parse_file(lines: &Vec<String>) -> Vec<Instruction> {
    let reg = Regex::new(r"^(...) ([-+\d]+)$").unwrap();
    let mut v = Vec::new();

    for l in lines {
        let l = reg.captures_iter(l).next().unwrap();
        let op = match &l[1] {
            "nop" => Opcode::Nop,
            "acc" => Opcode::Acc,
            "jmp" => Opcode::Jmp,
            _ => {
                panic!("unknown opcode found");
            }
        };
        let arg = l[2].parse::<i64>().unwrap();
        v.push(Instruction { op, arg });
    }

    v
}

fn step(op: &Opcode, arg: &i64) -> (i64, i64) {
    let mut acc = 0;
    let pc = match op {
        Opcode::Nop => 1,
        Opcode::Jmp => *arg,
        Opcode::Acc => {
            acc += arg;
            1
        }
    };

    (pc, acc)
}

fn execute(prog: &Vec<Instruction>, flip: bool) -> i64 {
    let mut acc = 0;
    let mut pc: i64 = 0;
    let mut visited = HashSet::<i64>::new();
    let mut jmpnop = Vec::<i64>::new();

    let mut inst = prog.get(pc as usize).unwrap();

    while !visited.contains(&pc) {
        visited.insert(pc);
        if inst.op == Opcode::Jmp || inst.op == Opcode::Nop {
            jmpnop.push(pc);
        }
        let (d_pc, d_acc) = step(&inst.op, &inst.arg);
        pc += d_pc;
        acc += d_acc;
        inst = prog.get(pc as usize).unwrap();
    }

    if !flip {
        return acc;
    }

    for jn in jmpnop {
        acc = 0;
        pc = 0;
        visited.clear();
        inst = prog.get(pc as usize).unwrap();
        while !visited.contains(&pc) {
            visited.insert(pc);
            let (d_pc, d_acc);
            if pc == jn {
                let flip = if inst.op == Opcode::Nop {
                    Opcode::Jmp
                } else {
                    Opcode::Nop
                };
                let x = step(&flip, &inst.arg);
                d_pc = x.0;
                d_acc = x.1;
            } else {
                let x = step(&inst.op, &inst.arg);
                d_pc = x.0;
                d_acc = x.1;
            }
            pc += d_pc;
            acc += d_acc;

            if pc as usize == prog.len() {
                return acc;
            }

            inst = prog.get(pc as usize).unwrap();
        }
    }

    acc
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_data() -> Vec<String> {
        let v = vec![
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        ];

        v
    }

    #[test]
    fn test1() {
        let v = parse_file(&test_data());
        let q = v.get(0).unwrap();
        assert_eq!(q.op, Opcode::Nop);
        assert_eq!(q.arg, 0);

        let r = v.get(5).unwrap();
        assert_eq!(r.op, Opcode::Acc);
        assert_eq!(r.arg, -99);
    }

    #[test]
    fn test2() {
        let prog = parse_file(&test_data());

        assert_eq!(execute(&prog, false), 5);
    }

    #[test]
    fn test3() {
        let prog = parse_file(&test_data());
        assert_eq!(execute(&prog, true), 8);
    }
}
