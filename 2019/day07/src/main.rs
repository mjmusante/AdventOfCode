use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use intcode::Computer;

/*
 * Code copy-n-pasted from https://rosettacode.org/wiki/Permutations but
 * modified to use i64.
 */
fn permutations(size: usize) -> Permutations {
    Permutations {
        idxs: (0..size as i64).collect(),
        swaps: vec![0; size],
        i: 0,
    }
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
                if self.i >= self.swaps.len() {
                    return None;
                }
                if self.swaps[self.i] < self.i {
                    break;
                }
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

    let mut part1 = 0;
    {
        let phase = permutations(5);

        for p in phase {
            let mut signal = 0;
            for i in p {
                let inputs = vec![i, signal];
                let mut c = Computer::new(&ary).with_input(inputs);
                c.run();
                signal = c.next_output();
                assert!(!c.has_output());
            }
            if signal > part1 {
                part1 = signal;
            }
        }
    }

    let mut part2 = 0;
    {
        let phase = permutations(5);
        for p in phase {
            let mut amps = vec![];
            for i in p {
                let mut c = Computer::new(&ary).with_input(vec![i + 5]);
                c.run();
                assert!(c.waiting_for_input() || c.halted());
                amps.push(c);
            }

            let mut signal = 0;

            // making the rash assumption that all amps stop at the same time
            while amps[0].waiting_for_input() {
                for amp in &mut amps {
                    amp.run_with_input(signal);
                    assert!(amp.waiting_for_input() || amp.halted());
                    signal = amp.next_output();
                }
            }

            if signal > part2 {
                part2 = signal;
            }
        }
    }

    println!("part 1 = {}", part1);
    println!("part 2 = {}", part2);
}
