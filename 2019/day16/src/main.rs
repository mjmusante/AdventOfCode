use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const SEQ: [i64; 4] = [0, 1, 0, -1];

fn next_base(elem: usize, offset: usize) -> i64 {
    let pos = (1 + offset) / (elem + 1);
    SEQ[pos % 4]
}

fn one_phase(sig: &Vec<i64>) -> Vec<i64> {
    let mut rslt = vec![];

    for digit in 0..sig.len() {
        let mut total = 0;
        for offset in 0..sig.len() {
            total += sig[offset] * next_base(digit, offset);
        }
        rslt.push(total.abs() % 10);
    }
    rslt
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
}
