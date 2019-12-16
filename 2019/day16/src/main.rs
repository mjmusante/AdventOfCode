use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::cmp::{max, min};

const SEQ: [i64; 4] = [0, 1, 0, -1];

fn next_base(elem: usize, offset: usize) -> i64 {
    let pos = (1 + offset) / (elem + 1);
    SEQ[pos % 4]
}

fn one_phase(sig: &Vec<i64>) -> Vec<i64> {
    let mut rslt = vec![];

    for digit in 0..sig.len() {
        let mut total = 0;
        for offset in digit..sig.len() {
            match next_base(digit, offset) {
                1 => {
                    total += sig[offset];
                }
                -1 => {
                    total -= sig[offset];
                }
                _ => (),
            }
        }
        rslt.push(total.abs() % 10);
    }
    rslt
}

fn fast_phase(sig: &std::vec::Vec<i64>, start: usize) -> Vec<i64> {
    let len = sig.len();
    let mut fresh = false;

    let mut digs = Vec::<i64>::with_capacity(len);
    digs.resize(len, 0);
    let mut total = 0;

    let max_neg = (len - 2) / 3;
    if start < max_neg {
        for digit in 0..max_neg {
            let pos_start = digit;
            let neg_start = 2 + digit * 3;
            let stride = 4 * (digit + 1);
            let mut pstride = pos_start;
            let mut nstride = neg_start;

            let mut total = 0;
            while pstride < len {
                for i in pstride..min(pstride + digit + 1, len) {
                    total += sig[i];
                }
                pstride += stride;
            }
            while nstride < len {
                for i in nstride..min(nstride + digit + 1, len) {
                    total -= sig[i];
                }
                nstride += stride;
            }
            digs[digit] = total.abs() % 10;
        }
    }

    for digit in max(max_neg, start)..len {
        if fresh {
            let last = digit * 2 - 1;
            total -= sig[digit - 1];
            if last < len {
                total += sig[last];
            }
            if last + 1 < len {
                total += sig[last + 1];
            }
        } else {
            fresh = true;
            total = 0;
            for i in digit..min(2 * digit + 1, len) {
                total += sig[i];
            }
        }
        digs[digit] = total.abs() % 10;
    }

    digs
}

fn main() {
    let f = File::open("inputs/day16.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let ary = vlist[0]
        .chars()
        .map(|num| num.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();

    let mut testsig = ary.clone();
    for _ in 0..100 {
        let rslt = one_phase(&testsig);
        testsig = rslt;
    }

    print!("part 1 = ");
    for i in 0..8 {
        print!("{}", testsig[i]);
    }
    println!("");

    let mut eighteight: usize = 0;
    for i in 0..7 {
        eighteight *= 10;
        eighteight += ary[i] as usize;
    }

    let mut longsig: Vec<i64> = vec![];
    for _ in 0..10_000 {
        longsig.extend(&ary);
    }

    for _ in 0..100 {
        let rslt = fast_phase(&longsig, eighteight);
        longsig = rslt;
    }
    print!("part 2 = ");
    for i in 0..8 {
        print!("{}", longsig[eighteight as usize + i]);
    }
    println!("");
}
