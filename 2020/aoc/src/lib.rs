use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Cannot find fine");
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
