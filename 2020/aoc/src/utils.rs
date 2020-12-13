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

    let mut t = (0, 1);
    let mut r = (mdl, val % mdl);
    while r.1 != 0 {
        let q = r.0 / r.1;
        t = (t.1, t.0 - q * t.1);
        r = (r.1, r.0 - q * r.1);
    }

    if r.0 > 1 {
        return None;
    }
    if t.0 < 0 {
        t.0 += mdl;
    }
    Some(t.0)
}
