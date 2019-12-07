use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use intcode::Computer;

/*
 * Code copy-n-pasted from https://rosettacode.org/wiki/Permutations but
 * modified to use i64.
 */
fn permutations(size: usize) -> Permutations {
    Permutations { idxs: (0..size as i64).collect(), swaps: vec![0; size], i: 0 }
}
 
struct Permutations {
    idxs: Vec<i64>,
    swaps: Vec<usize>,
    i: usize,
}
 
impl Iterator for Permutations {
    type Item = Vec<i64>;
 
    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() { return None; }
                if self.swaps[self.i] < self.i { break; }
                self.swaps[self.i] = 0;
                self.i += 1;
            }
            self.idxs.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.idxs.clone())
    }
}
/*
 * End copy-n-paste from https://rosettacode.org/wiki/Permutations
 */

fn main() {
    let f = File::open("inputs/day07.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let ary = vlist[0]
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    // let test_pgm = vec![3, 0, 3, 1, 102, 3, 0, 1, 2, 1, 2, 0, 4, 0, 99];
    let phase = permutations(5);

    let mut highest = 0;
    for p in phase {
        let mut signal = 0;
        for i in p {
            let inputs = [i, signal].to_vec();
            let mut c = Computer::new(&ary, inputs);
            signal = c.intcode();
        }
        if signal > highest {
            highest = signal;
        }
    }

    println!("result = {}", highest);
}
