use regex::Regex;
use std::collections::HashSet;

use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/08.txt");
    let prog = parse_file(&lines);

    println!("Part 1 = {}", execute(&prog));
}

#[derive(Debug, Eq, PartialEq)]
enum Opcode {
    Nop, Acc, Jmp
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
            _     => { panic!("unknown opcode found"); },
        };
        let arg = l[2].parse::<i64>().unwrap();
        v.push( Instruction{ op, arg } );
    }

    v
}

fn execute(prog: &Vec<Instruction>) -> i64 {
    let mut acc = 0;
    let mut pc : i64 = 0;
    let mut visited = HashSet::<i64>::new();

    let mut inst = prog.get(pc as usize).unwrap();

    while !visited.contains(&pc) {
        visited.insert(pc);
        // let oldpc = pc;
        pc = match inst.op {
            Opcode::Nop => pc + 1,
            Opcode::Acc => {
                acc += inst.arg;
                pc + 1
            },
            Opcode::Jmp => pc + inst.arg
        };
        // println!("{}: {:?} {} [acc = {}]", oldpc, inst.op, inst.arg, acc);
        inst = prog.get(pc as usize).unwrap();
    }

    acc
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let file = vec![
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
        let v = parse_file(&file);
        let q = v.get(0).unwrap();
        assert_eq!(q.op, Opcode::Nop);
        assert_eq!(q.arg, 0);

        let r = v.get(5).unwrap();
        assert_eq!(r.op, Opcode::Acc);
        assert_eq!(r.arg, -99);
    }

    #[test]
    fn test2() {
        let file = vec![
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
        let prog = parse_file(&file);

        assert_eq!(execute(&prog), 5);
    }
}
