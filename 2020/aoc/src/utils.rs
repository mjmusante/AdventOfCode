use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn records(filename: impl AsRef<Path>) -> Vec<String> {
    let data = read_to_string(filename).expect("Cannot find file");
    data.split("\n\n").map(|s| s.to_string()).collect()
}

pub fn lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Cannot find file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("Unable to read line"))
        .collect()
}

pub fn nums(filename: impl AsRef<Path>) -> Vec<i64> {
    let file = File::open(filename).expect("Cannot find file");
    let buf = BufReader::new(file);
    let mut v = Vec::new();

    for l in buf.lines().map(|line| line.unwrap()) {
        for num in l
            .split_whitespace()
            .map(|y| y.parse())
            .map(|n| n.ok().unwrap())
        {
            v.push(num)
        }
    }

    v
}

pub fn mod_inv(num: i128, modulus: i128) -> Option<i128> {
    let mdl = modulus.abs();
    let val = if num < 0 { mdl - (-num % mdl) } else { num };

    let mut t = 0;
    let mut nt = 1;
    let mut r = mdl;
    let mut nr = val % mdl;
    while nr != 0 {
        let q = r / nr;
        let tmp = nt;
        nt = t - q * nt;
        t = tmp;
        let tmp = nr;
        nr = r - q * nr;
        r = tmp;
    }

    if r > 1 {
        return None;
    }
    if t < 0 {
        t += modulus;
    }
    Some(t)
}
