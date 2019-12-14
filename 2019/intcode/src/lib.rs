use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(PartialEq)]
enum AddrMode {
    Positional,
    Immeidate,
    Relative,
    Mistake,
}

pub struct Computer {
    rollback: Vec<i64>,
    mem: Vec<i64>,
    ip: usize,
    input: Vec<i64>,
    output: Vec<i64>,
    running: bool,
    awaiting_input: bool,
    roffset: i64,
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            rollback: vec![],
            mem: vec![],
            ip: 0,
            input: vec![],
            output: vec![],
            running: true,
            awaiting_input: false,
            roffset: 0,
        }
    }
    pub fn from_array(mut self, program: &Vec<i64>) -> Computer {
        self.rollback = program.clone();
        self.reset();
        self
    }

    pub fn from_file(self, pathname: &str) -> Computer {
        let f = File::open(pathname).unwrap();
        let vlist = BufReader::new(&f)
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>();
        let ary = vlist[0]
            .split(",")
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        self.from_array(&ary)
    }

    pub fn with_input(mut self, input: Vec<i64>) -> Computer {
        assert!(self.input.len() == 0);
        self.input = input;
        self
    }

    pub fn reset(&mut self) {
        self.mem = self.rollback.clone();
        self.ip = 0;
        self.running = true;
        self.input = vec![];
        self.output = vec![];
        self.awaiting_input = false;
        self.roffset = 0;
    }

    pub fn set_noun_verb(&mut self, noun: i64, verb: i64) {
        self.mem[1] = noun;
        self.mem[2] = verb;
    }

    pub fn halted(&self) -> bool {
        !self.running
    }

    pub fn waiting_for_input(&self) -> bool {
        self.awaiting_input
    }

    pub fn has_output(&self) -> bool {
        self.output.len() > 0
    }

    pub fn next_output(&mut self) -> i64 {
        self.output.remove(0)
    }

    pub fn peek(&self, addr: usize) -> i64 {
        self.mem[addr]
    }

    pub fn poke(&mut self, addr: usize, val: i64) {
        self.mem[addr] = val;
    }

    fn to_mode(num: usize) -> AddrMode {
        match num {
            0 => AddrMode::Positional,
            1 => AddrMode::Immeidate,
            2 => AddrMode::Relative,
            _ => AddrMode::Mistake,
        }
    }

    fn get_opcode(&mut self) -> (usize, AddrMode, AddrMode, AddrMode) {
        let opval = self.mem[self.ip] as usize;
        self.ip += 1;
        (
            opval % 100,
            Computer::to_mode((opval / 100) % 10),
            Computer::to_mode((opval / 1000) % 10),
            Computer::to_mode((opval / 10000) % 10),
        )
    }

    fn read(&mut self, addr: usize) -> i64 {
        if self.mem.len() <= addr {
            self.mem.resize(addr + 1, 0);
        }
        self.mem[addr]
    }

    fn get_operand(&mut self, mode: AddrMode) -> i64 {
        let mut addr = self.ip;
        self.ip += 1;

        if mode == AddrMode::Relative {
            let adj = self.read(addr);
            return self.read((self.roffset + adj) as usize);
        }

        if mode == AddrMode::Positional {
            addr = self.read(addr) as usize;
        }
        self.read(addr)
    }

    fn get_dest(&mut self, mode: AddrMode) -> usize {
        let addr = self.ip;
        self.ip += 1;
        if mode == AddrMode::Relative {
            (self.roffset + self.read(addr)) as usize
        } else {
            self.read(addr) as usize
        }
    }

    fn write(&mut self, addr: usize, value: i64) {
        if self.mem.len() <= addr {
            self.mem.resize(addr + 1, 0);
        }
        self.mem[addr] = value;
    }

    pub fn run(&mut self) {
        while self.running {
            let (opcode, src1, src2, dst) = self.get_opcode();

            match opcode {
                1 => {
                    let op1 = self.get_operand(src1);
                    let op2 = self.get_operand(src2);
                    let dest = self.get_dest(dst);
                    self.write(dest, op1 + op2);
                }
                2 => {
                    let op1 = self.get_operand(src1);
                    let op2 = self.get_operand(src2);
                    let dest = self.get_dest(dst);
                    self.write(dest, op1 * op2);
                }
                3 => {
                    if self.input.len() == 0 {
                        self.ip -= 1;
                        self.awaiting_input = true;
                        return;
                    }
                    let dest = self.get_dest(src1);
                    let val = self.input.remove(0);
                    self.write(dest, val);
                }
                4 => {
                    let rslt = self.get_operand(src1);
                    self.output.push(rslt);
                }
                5 | 6 => {
                    let op1 = self.get_operand(src1);
                    let jump_loc = self.get_operand(src2) as usize;
                    if (opcode == 5 && op1 != 0) || (opcode == 6 && op1 == 0) {
                        self.ip = jump_loc;
                    }
                }
                7 | 8 => {
                    let op1 = self.get_operand(src1);
                    let op2 = self.get_operand(src2);
                    let dest = self.get_dest(dst);
                    self.write(
                        dest,
                        if (opcode == 7 && op1 < op2) || (opcode == 8 && op1 == op2) {
                            1
                        } else {
                            0
                        },
                    );
                }
                9 => {
                    self.roffset += self.get_operand(src1);
                }
                99 => {
                    self.running = false;
                }
                _ => {
                    println!("Invalid opcode {}", opcode);
                }
            };
        }
    }

    pub fn run_with_input(&mut self, inpval: i64) {
        self.input.push(inpval);
        self.awaiting_input = false;
        self.run();
    }

    pub fn current_ip(&self) -> usize {
        self.ip
    }
}

#[cfg(test)]
mod tests {
    use super::Computer;

    #[test]
    fn it_works() {
        let prg = vec![3, 0, 4, 0, 99];

        let mut c = Computer::new().from_array(&prg).with_input(vec![5]);
        assert_eq!(c.current_ip(), 0);
        c.run();
        assert_eq!(c.next_output(), 5);
    }

    #[test]
    fn addition() {
        let prg = vec![1, 0, 2, 0, 4, 0, 99];
        let mut c = Computer::new().from_array(&prg);
        c.run();
        assert_eq!(c.next_output(), 3);
    }

    #[test]
    fn multiplication() {
        let prg = vec![1002, 2, 3, 0, 4, 0, 99];
        let mut c = Computer::new().from_array(&prg);
        c.run();
        assert_eq!(c.next_output(), 9);
    }

    #[test]
    fn jump_positional() {
        let prg1 = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut c = Computer::new().from_array(&prg1).with_input(vec![0]);
        c.run();
        assert_eq!(c.next_output(), 0);
        c.reset();
        c.run_with_input(10);
        assert_eq!(c.next_output(), 1);
    }

    #[test]
    fn jump_immediate() {
        let prg1 = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];

        let mut c = Computer::new().from_array(&prg1).with_input(vec![0]);
        c.run();
        assert_eq!(c.next_output(), 0);

        c.reset();
        c.run_with_input(10);
        assert_eq!(c.next_output(), 1);
    }

    #[test]
    fn less_than() {
        let prg1 = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];

        let mut c = Computer::new().from_array(&prg1).with_input(vec![7]);
        c.run();
        assert_eq!(c.next_output(), 1);

        c.reset();
        c.run_with_input(8);
        assert_eq!(c.next_output(), 0);

        let prg2 = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];

        let mut c = Computer::new().from_array(&prg2).with_input(vec![7]);
        c.run();
        assert_eq!(c.next_output(), 1);

        c.reset();
        c.run_with_input(8);
        assert_eq!(c.next_output(), 0);
    }

    #[test]
    fn equals() {
        let prg1 = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];

        let mut c = Computer::new().from_array(&prg1).with_input(vec![7]);
        c.run();
        assert_eq!(c.next_output(), 0);

        c.reset();
        c.run_with_input(8);
        assert_eq!(c.next_output(), 1);

        let prg2 = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];

        let mut c = Computer::new().from_array(&prg2).with_input(vec![7]);
        c.run_with_input(7);
        assert_eq!(c.next_output(), 0);

        c.reset();
        c.run_with_input(8);
        assert_eq!(c.next_output(), 1);
    }

    #[test]
    fn compare_with_eight() {
        let prg = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        // check less than eight
        let mut c = Computer::new().from_array(&prg).with_input(vec![7]);
        c.run();
        assert_eq!(c.next_output(), 999);

        // check equal to eight
        c.reset();
        c.run_with_input(8);
        assert_eq!(c.next_output(), 1000);

        // check greater than eight
        c.reset();
        c.run_with_input(9);
        assert_eq!(c.next_output(), 1001);
    }

    #[test]
    fn quine() {
        let prg = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];

        let mut c = Computer::new().from_array(&prg);
        let mut output = vec![];
        while !c.halted() {
            c.run();
            while c.has_output() {
                output.push(c.next_output());
            }
        }
        assert_eq!(prg, output);
    }

    #[test]
    fn bignum1() {
        let prg = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut c = Computer::new().from_array(&prg);
        c.run();
        assert_eq!(1219070632396864, c.next_output());
    }

    #[test]
    fn bignum2() {
        let prg = vec![104, 1125899906842624, 99];
        let mut c = Computer::new().from_array(&prg);
        c.run();
        assert_eq!(1125899906842624, c.next_output());
    }
}
