use regex::Regex;
use std::collections::{HashMap, HashSet};
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

pub fn set_of_nums(filename: impl AsRef<Path>) -> HashSet<i64> {
    let file = File::open(filename).expect("cannot find file");
    let buf = BufReader::new(file);
    let mut hs = HashSet::new();

    for i in buf
        .lines()
        .map(|line| line.unwrap())
        .map(|num| num.parse().ok().unwrap())
    {
        hs.insert(i);
    }

    hs
}

pub fn parse_file(filename: impl AsRef<Path>, pattern: &str) -> Vec<HashMap<String, String>> {
    parse_vector(&lines(filename), pattern)
}

pub fn parse_vector(lines: &Vec<String>, pattern: &str) -> Vec<HashMap<String, String>> {
    let re = Regex::new(pattern).unwrap();
    let mut result = Vec::new();

    for l in lines {
        let caps = re.captures(&l).unwrap();
        let dict : HashMap<String, String> = re.capture_names().flatten()
            .filter_map(|grp| Some((grp.to_string(), caps.name(grp).unwrap().as_str().to_string())))
            .collect();
        result.push(dict);
    }

    result
}
