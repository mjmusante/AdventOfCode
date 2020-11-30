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
