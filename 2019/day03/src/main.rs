use std::process::exit;

use std::cmp::{min, max};

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use regex::Regex;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn mhatdist(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    fn clone(&self) -> Point {
        Point { x: self.x, y: self.y }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}

fn intersect(netlist: &Vec<Line>, line: &Line) -> Point {
    let mut result = Point { x: 0, y: 0 };
    let mut found = false;
    for n in netlist.iter() {
        if n.is_vertical() {
            if line.is_vertical() {
                continue;
            }
            let right = max(line.start.x, line.end.x);
            let left = min(line.start.x, line.end.x);
            if left > n.start.x || right < n.start.x {
                continue;
            }
            let top = max(n.start.y, n.end.y);
            let bot = min(n.start.y, n.end.y);
            if top < line.start.y || bot > line.start.y {
                continue;
            }
            let isect = Point { x: n.start.x, y: line.start.y };
            if !found || result.mhatdist() > isect.mhatdist() {
                found = true;
                result = isect.clone();
            }
        } else {
            if !line.is_vertical() {
                continue;
            }
            let top = max(line.start.y, line.end.y);
            let bot = min(line.start.y, line.end.y);
            if top < n.start.y || bot > n.start.y {
                continue;
            }
            let left = min(n.start.x, n.end.x);
            let right = max(n.start.x, n.end.x);
            if left > line.start.x || right < line.start.x {
                continue;
            }
            let isect = Point { x: line.start.x, y: n.start.y };
            if !found || result.mhatdist() > isect.mhatdist() {
                found = true;
                result = isect.clone();
            }
        }
    }

    result
}

fn main() {

    let f = File::open("data/input.txt").unwrap();
    let wires =  BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let first = &wires[0];
    let second = &wires[1];

    //let first = "R8,U5,L5,D3";
    //let second = "U7,R6,D4,L4";
    //let first = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
    //let second = "U62,R66,U55,R34,D71,R55,D58,R83";
    //let first = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
    //let second = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    
    let decode = Regex::new(r"([UDLR])(\d+)").unwrap();

    let mut curx = 0;
    let mut cury = 0;
    let mut netlist : Vec<Line> = Vec::new();

    for f in decode.captures_iter(first) {
        let dist = &f[2].parse::<i64>().unwrap();
        let start = Point { x: curx, y: cury };
        match &f[1] {
            "U" => { cury += dist; }
            "D" => { cury -= dist; }
            "L" => { curx -= dist; }
            "R" => { curx += dist; }
            _ => { println!("bad match"); exit(1); }
        };
        let end = Point { x: curx, y: cury };
        netlist.push(Line { start: start, end: end });
    }

    curx = 0;
    cury = 0;
    let mut closest = 0;

    for s in decode.captures_iter(second) {
        let dist = &s[2].parse::<i64>().unwrap();
        let start = Point { x: curx, y: cury };
        match &s[1] {
            "U" => { cury += dist; }
            "D" => { cury -= dist; }
            "L" => { curx -= dist; }
            "R" => { curx += dist; }
            _ => { println!("bad match"); exit(1); }
        };
        let end = Point { x: curx, y: cury };
        let myline = Line { start: start, end: end };
        let p = intersect(&netlist, &myline);
        let manhattan = p.x.abs() + p.y.abs();

        if closest == 0 || (manhattan > 0 && manhattan < closest) {
            closest = manhattan;
        }
    }

    println!("part 1: closest = {}", closest);
}
