use std::process::exit;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn intcode(program: &Vec<i64>, id: i64) -> i64 {
    let mut mem = program.clone();
    let mut pos = 0;

    loop {
        let opcode = mem[pos] % 100;
        let src1 = (mem[pos] / 100) % 10;
        let src2 = (mem[pos] / 1000) % 10;
        pos += 1;

        //println!("opcode = {}", opcode);

        let (val, loc) = match opcode {
            1 => {
                let op1 = if src1 == 0 { mem[mem[pos] as usize] } else { mem[pos] };
                let op2 = if src2 == 0 { mem[mem[pos + 1] as usize] } else { mem[pos + 1] };
                pos += 3;
                (op1 + op2, mem[pos - 1])
             }
            2 => {
                let op1 = if src1 == 0 { mem[mem[pos] as usize] } else { mem[pos] };
                let op2 = if src2 == 0 { mem[mem[pos + 1] as usize] } else { mem[pos + 1] };
                pos += 3;
                (op1 * op2, mem[pos - 1])
             }
            3 => { pos += 1; (id, mem[pos - 1]) }
            4 => {
                let op1 = if src1 == 0 { mem[mem[pos] as usize] } else { mem[pos] };
                pos += 1;
                if op1 != 0 {
                    return op1;
                }
                (0, -1)
             }
            5|6 => {
                let op1 = if src1 == 0 { mem[mem[pos] as usize] } else { mem[pos] };
                let op2 = if src2 == 0 { mem[mem[pos + 1] as usize] } else { mem[pos + 1] };
                if (opcode == 5 && op1 != 0) || (opcode == 6 && op1 == 0) {
                    pos = op2 as usize;
                } else {
                    pos += 2;
                }
                (0, -1)
             }
            7 => {
                let op1 = if src1 == 0 { mem[mem[pos] as usize] } else { mem[pos] };
                let op2 = if src2 == 0 { mem[mem[pos + 1] as usize] } else { mem[pos + 1] };
                pos += 3;
                if op1 < op2 {
                    (1, mem[pos - 1])
                } else {
                    (0, mem[pos - 1])
                }
             }
            8 => {
                let op1 = if src1 == 0 { mem[mem[pos] as usize] } else { mem[pos] };
                let op2 = if src2 == 0 { mem[mem[pos + 1] as usize] } else { mem[pos + 1] };
                pos += 3;
                if op1 == op2 {
                    (1, mem[pos - 1])
                } else {
                    (0, mem[pos - 1])
                }
             }
            99 => { (-1, -1) }
            _ => { return -1; }
        };

        if loc >= 0 {
            mem[loc as usize] = val
        }
        if val == -1 && loc == -1 {
            break;
        }
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

    /*
    let foo = "1002,4,3,4,33".split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    println!("test: {}", intcode(&foo));
    */

    println!("part 1 = {}", intcode(&ary, 1));
    println!("part 1 = {}", intcode(&ary, 5));
}
