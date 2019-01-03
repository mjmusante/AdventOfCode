use lines;
use regex::Regex;
use std::fmt;

pub fn run() -> (String, String) {
    let mut m = Machine::new();
    let lines = lines::lineread(String::from("puzzle_data/day16.txt"));
    let be_reg = Regex::new(r"^(\S*): +\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    let in_reg = Regex::new(r"^(\d+) (\d+) (\d+) (\d+)$").unwrap();

    let mut count = 0;
    let mut i = lines.iter();
    while let Some(l) = i.next() {
        if let Some(bcap) = be_reg.captures_iter(l).next() {
            assert_eq!(&bcap[1], "Before");
            let before = [
                bcap[2].parse::<i64>().unwrap(),
                bcap[3].parse::<i64>().unwrap(),
                bcap[4].parse::<i64>().unwrap(),
                bcap[5].parse::<i64>().unwrap(),
            ].to_vec();
            let opcap = in_reg.captures_iter(i.next().unwrap()).next().unwrap();
            let instruc = [
                opcap[1].parse::<i64>().unwrap(),
                opcap[2].parse::<i64>().unwrap(),
                opcap[3].parse::<i64>().unwrap(),
                opcap[4].parse::<i64>().unwrap(),
            ].to_vec();
            let acap = be_reg.captures_iter(i.next().unwrap()).next().unwrap();
            assert_eq!(&acap[1], "After");
            let after = [
                acap[2].parse::<i64>().unwrap(),
                acap[3].parse::<i64>().unwrap(),
                acap[4].parse::<i64>().unwrap(),
                acap[5].parse::<i64>().unwrap(),
            ].to_vec();
            if m.three_or_more(before, instruc, after) {
                count += 1;
            }
            i.next(); // throw away blank line
        } else {
            break;
        }
    }
    m.op.sort_by(|a, b| a.id.cmp(&b.id));

    i.next(); // one more blank line
    m.reg = [0,0,0,0].to_vec();
    while let Some(l) = i.next() {
        if let Some(cap) = in_reg.captures_iter(l).next() {
            let code = cap[1].parse::<usize>().unwrap();
            let src1 = cap[2].parse::<i64>().unwrap();
            let src2 = cap[3].parse::<i64>().unwrap();
            let dst = cap[4].parse::<i64>().unwrap();
            m.exec(&code, &src1, &src2, &dst);
        }
    }

    (format!("{}", count), format!("{}", m.reg[0]))
}


// -----

struct Machine {
    reg: Vec<i64>,
    op: Vec<Opcode>,
}

impl Machine {
    pub fn new() -> Machine {
        let mut v = vec![];

        v.push(Opcode::new("addr", addr));
        v.push(Opcode::new("addi", addi));
        v.push(Opcode::new("mulr", mulr));
        v.push(Opcode::new("muli", muli));
        v.push(Opcode::new("banr", banr));
        v.push(Opcode::new("bani", bani));
        v.push(Opcode::new("borr", borr));
        v.push(Opcode::new("bori", bori));
        v.push(Opcode::new("setr", setr));
        v.push(Opcode::new("seti", seti));
        v.push(Opcode::new("gtir", gtir));
        v.push(Opcode::new("gtri", gtri));
        v.push(Opcode::new("gtrr", gtrr));
        v.push(Opcode::new("eqir", eqir));
        v.push(Opcode::new("eqri", eqri));
        v.push(Opcode::new("eqrr", eqrr));

        Machine { reg: vec![0; 4], op: v }
    }

    pub fn exec(&mut self, code: &usize, src1: &i64, src2: &i64, dst: &i64) {
        (self.op[*code].func)(self, src1, src2, dst);
    }

    pub fn three_or_more(&mut self, before: Vec<i64>, inst: Vec<i64>, after: Vec<i64>) -> bool {
        let mut count = 0;

        let mut found = vec![];
        for i in 0..self.op.len() {
            self.reg = before.clone();
            (self.op[i].func)(self, &inst[1], &inst[2], &inst[3]);

            let mut last = 4;
            for i in 0..4 {
                if after[i] != self.reg[i] {
                    last = i;
                    break;
                }
            }
            if last == 4 {
                count += 1;
                found.push(i);
            }
        }

        let mut unknown = 0;
        let mut possible = 101;
        for i in 0..found.len() {
            if self.op[found[i]].id > 100 {
                unknown += 1;
                possible = found[i];
            }
        }
        if unknown == 1 {
            // println!("opcode {} can only be {}", inst[0], self.op[possible].name);
            self.op[possible].id = inst[0] as usize;
        }

        count >= 3
    }
}

#[derive(Clone)]
struct Opcode {
    name: String,
    id: usize,
    func: fn(m: &mut Machine, src1: &i64, src2: &i64, dst: &i64),
}

impl fmt::Debug for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{name:{}, id:{}}}", self.name, self.id)
    }
}

impl Opcode {
    pub fn new(name: &str, func: fn(m: &mut Machine, src1: &i64, src2: &i64, dst: &i64)) -> Opcode {
        Opcode { name: String::from(name), id: 987654321, func: func }
    }
}

fn addr(m: &mut Machine, src1: &i64, src2: &i64, dst: &i64) {
    m.reg[*dst as usize] = m.reg[*src1 as usize] + m.reg[*src2 as usize];
}

fn addi(m: &mut Machine, src1: &i64, src2: &i64, dst: &i64) {
    m.reg[*dst as usize] = m.reg[*src1 as usize] + *src2;
}

fn mulr(m: &mut Machine, src1: &i64, src2: &i64, dst: &i64) {
    m.reg[*dst as usize] = m.reg[*src1 as usize] * m.reg[*src2 as usize];
}

fn muli(m: &mut Machine, src1: &i64, src2: &i64, dst: &i64) {
    m.reg[*dst as usize] = m.reg[*src1 as usize] * *src2;
}

fn banr(m: &mut Machine, src1: &i64, src2: &i64, dst: &i64) {
    m.reg[*dst as usize] = m.reg[*src1 as usize] & m.reg[*src2 as usize];
}

fn bani(m: &mut Machine, src1: &i64, src2: &i64, dst: &i64) {
    m.reg[*dst as usize] = m.reg[*src1 as usize] & *src2;
}

fn borr(m: &mut Machine, src1: &i64, src2: &i64, dst: &i64) {
    m.reg[*dst as usize] = m.reg[*src1 as usize] | m.reg[*src2 as usize];
}

fn bori(m: &mut Machine, src1: &i64, src2: &i64, dst: &i64) {
    m.reg[*dst as usize] = m.reg[*src1 as usize] | *src2;
}

fn setr(m: &mut Machine, src1: &i64, _: &i64, dst: &i64) {
    m.reg[*dst as usize] = m.reg[*src1 as usize];
}

fn seti(m: &mut Machine, src1: &i64, _: &i64, dst: &i64) {
    m.reg[*dst as usize] = *src1;
}

fn gtir(m: &mut Machine, src1: &i64, src2: &i64, dst: &i64) {
    m.reg[*dst as usize] = match *src1 > m.reg[*src2 as usize] {
        true => 1,
        false => 0
    };
}

fn gtri(m: &mut Machine, src1: &i64, src2: &i64, dst: &i64) {
    m.reg[*dst as usize] = match m.reg[*src1 as usize] > *src2 {
        true => 1,
        false => 0
    };
}

fn gtrr(m: &mut Machine, src1: &i64, src2: &i64, dst: &i64) {
    m.reg[*dst as usize] = match m.reg[*src1 as usize] > m.reg[*src2 as usize] {
        true => 1,
        false => 0
    };
}

fn eqir(m: &mut Machine, src1: &i64, src2: &i64, dst: &i64) {
    m.reg[*dst as usize] = match *src1 == m.reg[*src2 as usize] {
        true => 1,
        false => 0
    };
}

fn eqri(m: &mut Machine, src1: &i64, src2: &i64, dst: &i64) {
    m.reg[*dst as usize] = match m.reg[*src1 as usize] == *src2 {
        true => 1,
        false => 0
    };
}

fn eqrr(m: &mut Machine, src1: &i64, src2: &i64, dst: &i64) {
    m.reg[*dst as usize] = match m.reg[*src1 as usize] == m.reg[*src2 as usize] {
        true => 1,
        false => 0
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_test1() {
        let mut m = Machine::new();

        m.exec(&1, &1, &2, &3);
        assert_eq!(m.reg[3], 2);

        m.exec(&0, &3, &3, &1);
        assert_eq!(m.reg[1], 4);
    }

    #[test]
    fn day16_test2() {
        let mut m = Machine::new();
        m.reg[0] = 3;
        m.reg[1] = 5;
        
        m.exec(&2, &0, &1, &2);
        assert_eq!(m.reg[2], 15);

        m.exec(&3, &0, &7, &2);
        assert_eq!(m.reg[2], 21);

        m.exec(&4, &0, &1, &2);
        assert_eq!(m.reg[2], 1);

        m.exec(&5, &0, &7, &2);
        assert_eq!(m.reg[2], 3);
    }

    #[test]
    fn day16_test3() {
        let mut m = Machine::new();

        assert!(m.three_or_more([3, 2, 1, 1].to_vec(), [9, 2, 1, 2].to_vec(), [3, 2, 2, 1].to_vec()));
        assert!(!m.three_or_more([20, 30, 40, 50].to_vec(),
            [9, 2, 1, 2].to_vec(),
            [2839, 29834, 9873, 928374].to_vec()));
    }
}
