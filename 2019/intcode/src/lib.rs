pub struct Computer {
    mem: Vec<i64>,
    ip: usize,
    input: Vec<i64>,
    running: bool,
}

impl Computer {
    pub fn new(program: &Vec<i64>, input: Vec<i64>) -> Computer {
        Computer {
            mem: program.clone(),
            ip: 0,
            input: input,
            running: true,
        }
    }

    pub fn set_noun_verb(&mut self, noun: i64, verb: i64) {
        self.mem[1] = noun;
        self.mem[2] = verb;
    }

    pub fn halted(&self) -> bool {
        !self.running
    }

    fn get_opcode(&mut self) -> (usize, bool, bool) {
        let opval = self.mem[self.ip] as usize;
        self.ip += 1;
        (
            opval % 100,
            (opval / 100) % 10 == 0,
            (opval / 1000) % 10 == 0,
        )
    }

    fn get_operand(&mut self, positional: bool) -> i64 {
        let mut addr = self.ip;
        if positional {
            addr = self.mem[addr] as usize;
        }
        self.ip += 1;
        self.mem[addr]
    }

    pub fn intcode(&mut self) -> i64 {
        while self.running {
            let (opcode, src1, src2) = self.get_opcode();

            match opcode {
                1 => {
                    let op1 = self.get_operand(src1);
                    let op2 = self.get_operand(src2);
                    let dest = self.get_operand(false) as usize;
                    self.mem[dest] = op1 + op2;
                }
                2 => {
                    let op1 = self.get_operand(src1);
                    let op2 = self.get_operand(src2);
                    let dest = self.get_operand(false) as usize;
                    self.mem[dest] = op1 * op2;
                }
                3 => {
                    let dest = self.get_operand(false) as usize;
                    self.mem[dest] = self.input.remove(0);
                }
                4 => {
                    let output = self.get_operand(src1);
                    // we peek ahead to see if we're halting
                    if self.mem[self.ip] == 99 {
                        self.running = false;
                    }
                    return output;
                }
                5 | 6 => {
                    let op1 = self.get_operand(src1);
                    let jump_loc = self.get_operand(src2) as usize;
                    if (opcode == 5 && op1 != 0) || (opcode == 6 && op1 == 0) {
                        self.ip = jump_loc;
                    }
                }
                7|8 => {
                    let op1 = self.get_operand(src1);
                    let op2 = self.get_operand(src2);
                    let dest = self.get_operand(false) as usize;
                    if (opcode == 7 &&op1 < op2) || (opcode == 8 && op1 == op2) {
                        self.mem[dest] = 1;
                    } else {
                        self.mem[dest] = 0;
                    }
                }
                99 => {
                    self.running = false;
                }
                _ => {
                    return -1;
                }
            };
        }

        self.mem[0]
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

        let mut c = Computer::new(&prg, [5].to_vec());
        assert_eq!(c.current_ip(), 0);
        assert_eq!(c.intcode(), 5);
    }

    #[test]
    fn addition() {
        let prg = vec![1, 0, 2, 0, 4, 0, 99];
        let mut c = Computer::new(&prg, [].to_vec());
        assert_eq!(c.intcode(), 3);
    }

    #[test]
    fn multiplication() {
        let prg = vec![1002, 2, 3, 0, 4, 0, 99];
        let mut c = Computer::new(&prg, [].to_vec());
        assert_eq!(c.intcode(), 9);
    }

    #[test]
    fn jump_positional() {
        let prg1 = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut c = Computer::new(&prg1, [0].to_vec());
        assert_eq!(c.intcode(), 0);
        let mut c = Computer::new(&prg1, [10].to_vec());
        assert_eq!(c.intcode(), 1);
    }

    #[test]
    fn jump_immediate() {
        let prg1 = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        let mut c = Computer::new(&prg1, [0].to_vec());
        assert_eq!(c.intcode(), 0);
        let mut c = Computer::new(&prg1, [10].to_vec());
        assert_eq!(c.intcode(), 1);
    }

    #[test]
    fn less_than() {
        let prg1 = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut c = Computer::new(&prg1, [7].to_vec());
        assert_eq!(c.intcode(), 1);
        let mut c = Computer::new(&prg1, [8].to_vec());
        assert_eq!(c.intcode(), 0);

        let prg2 = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let mut c = Computer::new(&prg2, [7].to_vec());
        assert_eq!(c.intcode(), 1);
        let mut c = Computer::new(&prg2, [8].to_vec());
        assert_eq!(c.intcode(), 0);
    }

    #[test]
    fn equals() {
        let prg1 = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut c = Computer::new(&prg1, [7].to_vec());
        assert_eq!(c.intcode(), 0);
        let mut c = Computer::new(&prg1, [8].to_vec());
        assert_eq!(c.intcode(), 1);

        let prg2 = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut c = Computer::new(&prg2, [7].to_vec());
        assert_eq!(c.intcode(), 0);
        let mut c = Computer::new(&prg2, [8].to_vec());
        assert_eq!(c.intcode(), 1);
    }

    #[test]
    fn compare_with_eight() {
        let prg = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        // check less than eight
        let mut c = Computer::new(&prg, [7].to_vec());
        assert_eq!(c.intcode(), 999);

        // check equao to eight
        let mut c = Computer::new(&prg, [8].to_vec());
        assert_eq!(c.intcode(), 1000);

        // check greater than eight
        let mut c = Computer::new(&prg, [9].to_vec());
        assert_eq!(c.intcode(), 1001);
    }
}
